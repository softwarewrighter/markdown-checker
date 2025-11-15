use std::fs;
use std::io;
use std::path::Path;

/// Read file content and validate it's UTF-8
pub fn read_file_content(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Write content to file
pub fn write_file_content(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_existing_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test content").unwrap();

        let content = read_file_content(temp_file.path()).unwrap();
        assert_eq!(content.trim(), "Test content");
    }

    #[test]
    fn test_file_not_found() {
        let result = read_file_content(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_invalid_utf8() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Write invalid UTF-8 bytes
        temp_file.write_all(&[0xFF, 0xFE, 0xFD]).unwrap();

        let result = read_file_content(temp_file.path());
        assert!(result.is_err());
    }
}
