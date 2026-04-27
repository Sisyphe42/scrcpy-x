//! Binary resolution module
//! 
//! Handles finding ADB and scrcpy binaries on the system.
//! Supports three resolution modes: Auto (PATH), Bundled, and Custom (user-specified).

mod bundled;

use std::path::PathBuf;
use which::which;
use crate::settings::{BinarySource, load_settings};

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

/// Resolve a custom path from settings if available and valid.
fn custom_path_for_binary(binary_type: BinaryType, adb_path: Option<String>, scrcpy_path: Option<String>) -> Option<PathBuf> {
    let path_str = match binary_type {
        BinaryType::Adb => adb_path,
        BinaryType::Scrcpy => scrcpy_path,
    };
    path_str.filter(|p| !p.is_empty()).map(PathBuf::from).filter(|p| p.exists() && p.is_file())
}

/// Find ADB binary respecting the user's binary source preference.
///
/// Resolution order depends on `adb_source` setting:
/// - **Auto**: PATH → Bundled
/// - **Bundled**: Bundled only (fails if unavailable)
/// - **Custom**: User-specified path only
pub fn find_adb() -> Result<PathBuf, String> {
    let settings = load_settings().ok();
    let source = settings.as_ref().map(|s| &s.binary_config.adb_source);
    let adb_path = settings.as_ref().and_then(|s| s.binary_config.adb_path.clone());

    match source {
        Some(BinarySource::Custom) => {
            custom_path_for_binary(BinaryType::Adb, adb_path, None)
                .ok_or_else(|| "Custom ADB path not set or file not found. Please configure in Settings.".to_string())
        }
        Some(BinarySource::Bundled) => {
            bundled::get_bundled_adb_path()
                .filter(|p| p.exists())
                .ok_or_else(|| "Bundled ADB not available. Switch to Auto or Custom in Settings.".to_string())
        }
        _ => {
            // Auto (default): PATH → Bundled
            if let Some(path) = find_in_path(BinaryType::Adb) {
                return Ok(path);
            }
            if let Some(path) = bundled::get_bundled_adb_path() {
                if path.exists() {
                    return Ok(path);
                }
            }
            Err("ADB not found. Please install Android SDK Platform Tools and ensure adb is in PATH.".to_string())
        }
    }
}

/// Find scrcpy binary respecting the user's binary source preference.
///
/// Same resolution logic as `find_adb`, but uses `scrcpy_source` setting.
pub fn find_scrcpy() -> Result<PathBuf, String> {
    let settings = load_settings().ok();
    let source = settings.as_ref().map(|s| &s.binary_config.scrcpy_source);
    let scrcpy_path = settings.as_ref().and_then(|s| s.binary_config.scrcpy_path.clone());

    match source {
        Some(BinarySource::Custom) => {
            custom_path_for_binary(BinaryType::Scrcpy, None, scrcpy_path)
                .ok_or_else(|| "Custom scrcpy path not set or file not found. Please configure in Settings.".to_string())
        }
        Some(BinarySource::Bundled) => {
            bundled::get_bundled_scrcpy_path()
                .filter(|p| p.exists())
                .ok_or_else(|| "Bundled scrcpy not available. Switch to Auto or Custom in Settings.".to_string())
        }
        _ => {
            // Auto (default): PATH → Bundled
            if let Some(path) = find_in_path(BinaryType::Scrcpy) {
                return Ok(path);
            }
            if let Some(path) = bundled::get_bundled_scrcpy_path() {
                if path.exists() {
                    return Ok(path);
                }
            }
            Err("scrcpy not found. Please install scrcpy and ensure it is in PATH.".to_string())
        }
    }
}

/// Check if a binary exists and is executable
pub fn binary_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}

/// Check if bundled binaries are available (for UI display)
pub fn has_bundled_binaries() -> bool {
    bundled::get_bundled_adb_path().is_some() && bundled::get_bundled_scrcpy_path().is_some()
}

/// Check current binary detection status (for UI display)
pub fn detect_binaries() -> (Option<String>, Option<String>) {
    let adb = find_in_path(BinaryType::Adb)
        .or_else(|| bundled::get_bundled_adb_path().filter(|p| p.exists()))
        .map(|p| p.to_string_lossy().to_string());
    let scrcpy = find_in_path(BinaryType::Scrcpy)
        .or_else(|| bundled::get_bundled_scrcpy_path().filter(|p| p.exists()))
        .map(|p| p.to_string_lossy().to_string());
    (adb, scrcpy)
}
