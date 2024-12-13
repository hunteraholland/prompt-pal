use std::path::PathBuf;
use walkdir::WalkDir;

/// Scans a directory and returns a list of all files found.
///
/// # Arguments
/// * `target_dir` - The directory path to scan
///
/// # Returns
/// * `Vec<PathBuf>` - A vector of paths to all files found
///
/// # Example
/// ```rust
/// use promptpal::scan_directory;
/// # use std::io;
/// # fn main() -> io::Result<()> {
/// let files = scan_directory(".")?;
/// for file in files {
///     println!("{}", file.display());
/// }
/// # Ok(())
/// # }
/// ```
#[allow(dead_code)]
pub fn scan_directory(target_dir: impl Into<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
    let target_dir = target_dir.into();
    let mut files = Vec::new();

    // Validate that the directory exists and is a directory
    if !target_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }
    if !target_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

    // Walk the directory tree
    for entry in WalkDir::new(&target_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io;
    use tempfile::tempdir;

    #[test]
    fn test_scan_directory() -> io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create some test files
        File::create(temp_path.join("file1.txt"))?;
        File::create(temp_path.join("file2.txt"))?;

        // Create a subdirectory with a file
        fs::create_dir(temp_path.join("subdir"))?;
        File::create(temp_path.join("subdir").join("file3.txt"))?;

        // Scan the directory
        let files = scan_directory(temp_path)?;

        // Verify we found all files
        assert_eq!(files.len(), 3);
        assert!(files.iter().any(|p| p.ends_with("file1.txt")));
        assert!(files.iter().any(|p| p.ends_with("file2.txt")));
        assert!(files.iter().any(|p| p.ends_with("file3.txt")));

        Ok(())
    }

    #[test]
    fn test_scan_nonexistent_directory() {
        let result = scan_directory("nonexistent_directory");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }
}
