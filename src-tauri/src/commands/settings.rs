//! Settings management commands
//!
//! Commands for managing application settings.

use crate::settings as settings_mod;
use crate::binaries::{find_adb, find_scrcpy};
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

/// Get ADB path
#[tauri::command]
pub async fn get_adb_path() -> Result<Option<String>, String> {
    match find_adb() {
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
        Err(_) => Ok(None),
    }
}

/// Get scrcpy path
#[tauri::command]
pub async fn get_scrcpy_path() -> Result<Option<String>, String> {
    match find_scrcpy() {
        Ok(path) => Ok(Some(path.to_string_lossy().to_string())),
        Err(_) => Ok(None),
    }
}
