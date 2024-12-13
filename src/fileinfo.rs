use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Represents metadata and content information for a file
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Path to the file
    pub path: PathBuf,
    /// Size of the file in bytes
    pub size: u64,
    /// Optional preview of the file's content (first few bytes)
    pub content_preview: Option<String>,
}

impl FileInfo {
    /// Creates a new FileInfo instance from a path
    ///
    /// # Arguments
    /// * `path` - Path to the file to analyze
    /// * `preview_length` - Number of bytes to read for the content preview (0 for no preview)
    ///
    /// # Returns
    /// * `io::Result<FileInfo>` - File information or an error
    pub fn new(path: impl AsRef<Path>, preview_length: usize) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let content_preview = if preview_length > 0 {
            Some(Self::read_preview(&path, preview_length)?)
        } else {
            None
        };

        Ok(FileInfo {
            path,
            size,
            content_preview,
        })
    }

    /// Reads a preview of the file's content
    fn read_preview(path: &Path, preview_length: usize) -> io::Result<String> {
        let mut file = fs::File::open(path)?;
        let mut buffer = vec![0; preview_length];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        
        // Try to convert to UTF-8, fall back to displaying as hex if invalid
        String::from_utf8(buffer.clone())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
            .or_else(|_| {
                Ok(buffer
                    .iter()
                    .take(preview_length)
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" "))
            })
    }
}

/// Gathers file information for all files in a directory
///
/// # Arguments
/// * `target_dir` - Directory to scan
/// * `preview_length` - Number of bytes to read for content previews (0 for no previews)
///
/// # Returns
/// * `io::Result<Vec<FileInfo>>` - Vector of file information or an error
pub fn gather_file_info(
    target_dir: impl AsRef<Path>,
    preview_length: usize,
) -> io::Result<Vec<FileInfo>> {
    let paths = crate::walkdir::scan_directory(target_dir.as_ref().to_path_buf())?;
    
    let mut file_infos = Vec::new();
    for path in paths {
        match FileInfo::new(&path, preview_length) {
            Ok(info) => file_infos.push(info),
            Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
        }
    }
    
    Ok(file_infos)
} 