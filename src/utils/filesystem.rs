use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("Failed to create directory: {0}")]
    CreateDir(String),
    #[error("Failed to read file: {0}")]
    ReadFile(String),
    #[error("Failed to write file: {0}")]
    WriteFile(String),
    #[error("Failed to delete file/directory: {0}")]
    Delete(String),
}

/// Creates a directory and its parent directories if they don't exist.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FsError> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)
            .map_err(|_| FsError::CreateDir(path.to_string_lossy().to_string()))?;
    }
    Ok(())
}

/// Reads a file to a string.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, FsError> {
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|_| FsError::ReadFile(path.to_string_lossy().to_string()))
}

/// Writes a string to a file.
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), FsError> {
    let path = path.as_ref();
    fs::write(path, content).map_err(|_| FsError::WriteFile(path.to_string_lossy().to_string()))
}

/// Safely removes a file if it exists.
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), FsError> {
    let path = path.as_ref();
    if path.exists() {
        fs::remove_file(path).map_err(|_| FsError::Delete(path.to_string_lossy().to_string()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_ops() {
        let test_dir = ".fs_test_dir";
        let test_file = format!("{test_dir}/test.txt");

        assert!(create_dir_all(test_dir).is_ok());
        assert!(write_file(&test_file, "hello rust").is_ok());

        let content = read_file(&test_file).unwrap();
        assert_eq!(content, "hello rust");

        assert!(remove_file(&test_file).is_ok());
        assert!(!Path::new(&test_file).exists());

        fs::remove_dir(test_dir).unwrap();
    }
}

