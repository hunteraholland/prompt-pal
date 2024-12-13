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
    /// File content (preview or full)
    pub content: Option<String>,
    /// Whether the content is complete or just a preview
    pub is_content_complete: bool,
}

impl FileInfo {
    /// Creates a new FileInfo instance with a preview of the content
    pub fn with_preview(path: impl AsRef<Path>, preview_length: usize) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let (content, is_complete) = if preview_length > 0 {
            let content = Self::read_content(&path, Some(preview_length))?;
            let is_complete = preview_length >= size as usize;
            (Some(content), is_complete)
        } else {
            (None, false)
        };

        Ok(FileInfo {
            path,
            size,
            content,
            is_content_complete: is_complete,
        })
    }

    /// Creates a new FileInfo instance with the complete file content
    pub fn with_full_content(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let content = Some(Self::read_content(&path, None)?);

        Ok(FileInfo {
            path,
            size,
            content,
            is_content_complete: true,
        })
    }

    /// Reads content from a file, either preview or full
    fn read_content(path: &Path, max_length: Option<usize>) -> io::Result<String> {
        let mut file = fs::File::open(path)?;
        let file_size = file.metadata()?.len() as usize;
        let read_size = if let Some(max) = max_length {
            file_size.min(max)
        } else {
            file_size
        };

        let mut buffer = Vec::with_capacity(read_size);
        let mut chunk = vec![0; 8192]; // 8KB chunks

        while buffer.len() < read_size {
            let remaining = read_size - buffer.len();
            let chunk_size = remaining.min(chunk.len());
            let bytes_read = file.read(&mut chunk[..chunk_size])?;
            if bytes_read == 0 {
                break; // EOF
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        let buffer_clone = buffer.clone();
        String::from_utf8(buffer)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
            .or_else(|_| {
                Ok(buffer_clone
                    .iter()
                    .take(read_size)
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
        match FileInfo::with_preview(&path, preview_length) {
            Ok(info) => file_infos.push(info),
            Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
        }
    }

    Ok(file_infos)
}
