use promptpal::walkdir::scan_directory;
use std::fs::{self, File};
use std::io;
use tempfile::tempdir;

#[test]
fn test_directory_scanning_integration() -> io::Result<()> {
    // Create a more complex directory structure for integration testing
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // Create files in root
    File::create(temp_path.join("root1.txt"))?;
    File::create(temp_path.join("root2.md"))?;

    // Create nested directory structure
    let nested_dir = temp_path.join("nested");
    fs::create_dir(&nested_dir)?;
    File::create(nested_dir.join("nested1.txt"))?;
    
    // Create deep nested structure
    let deep_dir = nested_dir.join("deep");
    fs::create_dir(&deep_dir)?;
    File::create(deep_dir.join("deep1.rs"))?;

    // Test scanning
    let files = scan_directory(temp_path)?;

    // Verify file count
    assert_eq!(files.len(), 4, "Should find exactly 4 files");

    // Verify file extensions
    let extensions: Vec<_> = files
        .iter()
        .filter_map(|p| p.extension())
        .map(|ext| ext.to_string_lossy().to_string())
        .collect();

    assert!(extensions.contains(&"txt".to_string()));
    assert!(extensions.contains(&"md".to_string()));
    assert!(extensions.contains(&"rs".to_string()));

    Ok(())
} 