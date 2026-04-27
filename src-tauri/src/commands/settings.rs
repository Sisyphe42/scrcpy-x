//! Settings management commands
//!
//! Commands for managing application settings.

use crate::settings as settings_mod;
use crate::binaries::{find_adb, find_scrcpy, detect_binaries, has_bundled_binaries};
use serde::{Deserialize, Serialize};

use crate::settings::AppSettings;

/// Get application settings
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    settings_mod::load_settings()
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(settings: AppSettings) -> Result<(), String> {
    settings_mod::save_settings(settings)
}

/// Get ADB path (auto-detected)
#[tauri::command]
pub async fn get_adb_path() -> Result<Option<String>, String> {
    match find_adb() {
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
        Err(_) => Ok(None),
    }
}

/// Get scrcpy path (auto-detected)
#[tauri::command]
pub async fn get_scrcpy_path() -> Result<Option<String>, String> {
    match find_scrcpy() {
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
        Err(_) => Ok(None),
    }
}

/// Binary detection status for UI
#[derive(Debug, Serialize)]
pub struct BinaryStatus {
    pub adb_path: Option<String>,
    pub scrcpy_path: Option<String>,
    pub bundled_available: bool,
}

/// Detect binary status (for Settings UI display)
#[tauri::command]
pub async fn get_binary_status() -> Result<BinaryStatus, String> {
    let (adb, scrcpy) = detect_binaries();
    Ok(BinaryStatus {
        adb_path: adb,
        scrcpy_path: scrcpy,
        bundled_available: has_bundled_binaries(),
    })
}
