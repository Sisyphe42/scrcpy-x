//! Settings management commands
//! 
//! Commands for managing application settings.

use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Last used profile
    pub last_profile: Option<String>,
    /// Window bounds
    pub window_bounds: Option<WindowBounds>,
    /// Binary paths
    pub binary_paths: BinaryPaths,
    /// Theme setting
    pub theme: String,
    /// Max concurrent sessions
    pub max_sessions: u8,
}

/// Window position and size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Binary paths configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryPaths {
    pub adb: Option<String>,
    pub scrcpy: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            last_profile: None,
            window_bounds: None,
            binary_paths: BinaryPaths {
                adb: None,
                scrcpy: None,
            },
            theme: "system".to_string(),
            max_sessions: 5,
        }
    }
}

/// Get application settings
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    // TODO: Implement settings load
    Ok(AppSettings::default())
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(settings: AppSettings) -> Result<(), String> {
    // TODO: Implement settings save
    Ok(())
}

/// Get ADB path
#[tauri::command]
pub async fn get_adb_path() -> Result<Option<String>, String> {
    // TODO: Implement ADB path resolution
    Ok(None)
}

/// Get scrcpy path
#[tauri::command]
pub async fn get_scrcpy_path() -> Result<Option<String>, String> {
    // TODO: Implement scrcpy path resolution
    Ok(None)
}
