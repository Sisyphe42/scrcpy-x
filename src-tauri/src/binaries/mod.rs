//! Binary resolution module
//! 
//! Handles finding ADB and scrcpy binaries on the system.

mod bundled;

use std::path::PathBuf;
use which::which;

/// Binary type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryType {
    Adb,
    Scrcpy,
}

impl BinaryType {
    /// Get the binary name for the current platform
    pub fn binary_name(&self) -> &'static str {
        #[cfg(target_os = "windows")]
        {
            match self {
                BinaryType::Adb => "adb.exe",
                BinaryType::Scrcpy => "scrcpy.exe",
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            match self {
                BinaryType::Adb => "adb",
                BinaryType::Scrcpy => "scrcpy",
            }
        }
    }
}

/// Find a binary in PATH
pub fn find_in_path(binary_type: BinaryType) -> Option<PathBuf> {
    which(binary_type.binary_name()).ok()
}

/// Find ADB binary
/// 
/// Resolution order:
/// 1. System PATH
/// 2. Bundled binary (if available)
pub fn find_adb() -> Result<PathBuf, String> {
    // Try system PATH first
    if let Some(path) = find_in_path(BinaryType::Adb) {
        return Ok(path);
    }
    
    // Try bundled binary
    if let Some(path) = bundled::get_bundled_adb_path() {
        if path.exists() {
            return Ok(path);
        }
    }
    
    Err("ADB not found. Please install Android SDK Platform Tools and ensure adb is in PATH.".to_string())
}

/// Find scrcpy binary
/// 
/// Resolution order:
/// 1. System PATH
/// 2. Bundled binary (if available)
pub fn find_scrcpy() -> Result<PathBuf, String> {
    // Try system PATH first
    if let Some(path) = find_in_path(BinaryType::Scrcpy) {
        return Ok(path);
    }
    
    // Try bundled binary
    if let Some(path) = bundled::get_bundled_scrcpy_path() {
        if path.exists() {
            return Ok(path);
        }
    }
    
    Err("scrcpy not found. Please install scrcpy and ensure it is in PATH.".to_string())
}

/// Check if a binary exists and is executable
pub fn binary_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_names() {
        #[cfg(target_os = "windows")]
        {
            assert_eq!(BinaryType::Adb.binary_name(), "adb.exe");
            assert_eq!(BinaryType::Scrcpy.binary_name(), "scrcpy.exe");
        }
        #[cfg(not(target_os = "windows"))]
        {
            assert_eq!(BinaryType::Adb.binary_name(), "adb");
            assert_eq!(BinaryType::Scrcpy.binary_name(), "scrcpy");
        }
    }
}
