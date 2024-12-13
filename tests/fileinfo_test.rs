use promptpal::fileinfo::{gather_file_info, FileInfo};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_file_info_creation() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    let test_file_path = temp_dir.path().join("test.txt");
    let test_content = "Hello, World!";

    // Create a test file with known content
    let mut file = File::create(&test_file_path)?;
    file.write_all(test_content.as_bytes())?;

    // Test FileInfo creation with preview
    let file_info = FileInfo::new(&test_file_path, 5)?;
    assert_eq!(file_info.size, 13); // Length of "Hello, World!"
    assert_eq!(file_info.content_preview, Some("Hello".to_string()));

    // Test FileInfo creation without preview
    let file_info_no_preview = FileInfo::new(&test_file_path, 0)?;
    assert_eq!(file_info_no_preview.size, 13);
    assert_eq!(file_info_no_preview.content_preview, None);

    Ok(())
}

#[test]
fn test_gather_file_info() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // Create test files with different content
    create_test_file(temp_path, "file1.txt", "Content 1")?;
    create_test_file(temp_path, "file2.txt", "Content 2")?;

    // Create a subdirectory with a file
    let subdir = temp_path.join("subdir");
    fs::create_dir(&subdir)?;
    create_test_file(&subdir, "file3.txt", "Content 3")?;

    // Gather file info with previews
    let file_infos = gather_file_info(temp_path, 5)?;

    // Verify we found all files
    assert_eq!(file_infos.len(), 3);

    // Verify each file has correct metadata
    for info in file_infos {
        assert!(info.size > 0);
        assert!(info.content_preview.is_some());
        assert!(info.path.exists());
    }

    Ok(())
}

#[test]
fn test_binary_file_preview() -> std::io::Result<()> {
    let temp_dir = tempdir()?;
    let test_file_path = temp_dir.path().join("binary.bin");

    // Create a binary file with non-UTF8 content
    let binary_content = [0xFF, 0x00, 0xFE, 0x12];
    let mut file = File::create(&test_file_path)?;
    file.write_all(&binary_content)?;

    // Test FileInfo creation with preview
    let file_info = FileInfo::new(&test_file_path, 4)?;
    assert_eq!(file_info.size, 4);
    // Preview should be hex representation
    assert_eq!(file_info.content_preview, Some("ff 00 fe 12".to_string()));

    Ok(())
}

// Helper function to create test files
fn create_test_file(dir: &Path, name: &str, content: &str) -> std::io::Result<()> {
    let path = dir.join(name);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}
