use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// Helpers -----------------------------------------------------------------
fn app_data_dir_fallback() -> Option<PathBuf> {
    // Windows-style APPDATA
    if let Ok(base) = std::env::var("APPDATA") {
        return Some(PathBuf::from(base).join("scrcpyx"));
    }
    // POSIX-style home directory
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home).join(".local").join("share").join("scrcpyx"));
    }
    None
}

fn settings_path() -> Result<PathBuf, String> {
    let base = app_data_dir_fallback().ok_or("App data dir not found".to_string())?;
    // Ensure directory exists
    if !base.exists() {
        if let Err(e) = fs::create_dir_all(&base) {
            return Err(format!("Failed to create app data dir: {}", e));
        }
    }
    Ok(base.join("settings.json"))
}

// Data models --------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BinaryPaths {
    pub adb: Option<String>,
    pub scrcpy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub last_profile: Option<String>,
    pub window_bounds: Option<WindowBounds>,
    pub binary_paths: BinaryPaths,
    pub theme: String,
    pub max_sessions: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            last_profile: None,
            window_bounds: None,
            binary_paths: BinaryPaths::default(),
            theme: "system".to_string(),
            max_sessions: 5,
        }
    }
}

// API ----------------------------------------------------------------------
pub fn load_settings() -> Result<AppSettings, String> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let mut s = String::new();
    File::open(&path)
        .map_err(|e| format!("Open: {}", e))?
        .read_to_string(&mut s)
        .map_err(|e| format!("Read: {}", e))?;
    // Parse JSON with defaults for missing fields
    let mut settings: AppSettings = serde_json::from_str(&s).map_err(|e| format!("Parse: {}", e))?;
    // Fill in sensible defaults for potentially missing fields
    if settings.theme.is_empty() {
        settings.theme = "system".to_string();
    }
    if settings.max_sessions == 0 {
        settings.max_sessions = 5;
    }
    // last_profile, window_bounds, and binary_paths are optional and already handled by types
    if settings.binary_paths.adb.is_none() && settings.binary_paths.scrcpy.is_none() {
        settings.binary_paths = BinaryPaths::default();
    }
    Ok(settings)
}

pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    let path = settings_path()?;
    let temp_path = path.with_extension("json.tmp");

    // Serialize
    let data = serde_json::to_string_pretty(&settings).map_err(|e| format!("Serialize error: {}", e))?;
    // Atomic write to temp file
    fs::write(&temp_path, data).map_err(|e| format!("Temp write failed: {}", e))?;
    // Cleanup existing destination if any
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Cleanup existing file: {}", e))?;
    }
    fs::rename(&temp_path, &path).map_err(|e| format!("Atomic rename failed: {}", e))?;
    Ok(())
}

// Types are already public, no re-exports needed
