//! Utils module for pocsuite-rs

use std::fs;
use std::path::Path;
use std::io::{self, Read, Write};

/// Read content from a file
pub fn read_file(path: &Path) -> io::Result<String> {
    let mut content = String::new();
    fs::File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

/// Write content to a file
pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?.write_all(content.as_bytes())
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Get file extension
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}