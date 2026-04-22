//! Profile storage backend
//!
//! Handles saving, loading, listing and deleting profiles with atomic file writes
//! and app data directory storage. Profiles are stored as JSON files under
//! {app_data_dir}/profiles/{name}.json. The default profile name is tracked in
//! {app_data_dir}/default_profile.txt.

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::session::SessionOptions;

/// Profile information (mirrors TypeScript interface)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub is_default: bool,
    pub options: SessionOptions,
}

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

fn profiles_dir() -> Result<PathBuf, String> {
    // app data directory using a safe fallback (Taura API may be unavailable in some builds)
    let base = app_data_dir_fallback().ok_or("App data dir not found".to_string())?;
    let dir = base.join("profiles");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed create profiles dir: {}", e))?;
    }
    Ok(dir)
}

fn default_profile_path() -> Result<PathBuf, String> {
    let base = app_data_dir_fallback().ok_or("App data dir not found".to_string())?;
    Ok(base.join("default_profile.txt"))
}

fn read_default_name() -> Result<Option<String>, String> {
    let path = default_profile_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let mut s = String::new();
    File::open(&path).map_err(|e| format!("Open default: {}", e))?.read_to_string(&mut s).map_err(|e| format!("Read default: {}", e))?;
    Ok(Some(s.trim().to_string()))
}

// API -----------------------------------------------------------------------
pub async fn save_profile(profile: Profile) -> Result<(), String> {
    let dir = profiles_dir()?;
    let path = dir.join(format!("{}.json", profile.name));
    let temp_path = dir.join(format!("{}.json.tmp", profile.name));

    // Serialize
    let data = serde_json::to_string_pretty(&profile).map_err(|e| format!("Serialize error: {}", e))?;
    // Atomic write to temp file
    fs::write(&temp_path, data).map_err(|e| format!("Temp write failed: {}", e))?;
    // Remove existing destination if any (to allow atomic rename)
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Cleanup existing file: {}", e))?;
    }
    fs::rename(&temp_path, &path).map_err(|e| format!("Atomic rename failed: {}", e))?;

    // If this is the default, update default_profile.txt
    if profile.is_default {
        let mut f = File::create(default_profile_path()?).map_err(|e| format!("Default write: {}", e))?;
        f.write_all(profile.name.as_bytes()).map_err(|e| format!("Default write: {}", e))?;
    }

    Ok(())
}

pub async fn load_profile(name: String) -> Result<Profile, String> {
    let dir = profiles_dir()?;
    let path = dir.join(format!("{}.json", name));
    if !path.exists() {
        return Err("Profile not found".to_string());
    }
    let mut s = String::new();
    File::open(&path).map_err(|e| format!("Open: {}", e))?.read_to_string(&mut s).map_err(|e| format!("Read: {}", e))?;
    let mut profile: Profile = serde_json::from_str(&s).map_err(|e| format!("Parse: {}", e))?;
    // Determine default status from stored default file
    if let Ok(Some(def)) = read_default_name() {
        profile.is_default = def == name;
    }
    Ok(profile)
}

pub async fn list_profiles() -> Result<Vec<Profile>, String> {
    let dir = profiles_dir()?;
    let mut result: Vec<Profile> = Vec::new();
    let def_name = read_default_name()?.unwrap_or_default();
    for entry in fs::read_dir(&dir).map_err(|e| format!("Read dir: {}", e))? {
        let entry = entry.map_err(|e| format!("Dir entry: {}", e))?;
        if entry.file_type().map_err(|e| format!("FT: {}", e))?.is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    let mut s = String::new();
                    File::open(entry.path()).map_err(|e| format!("Open: {}", e))?.read_to_string(&mut s).map_err(|e| format!("Read: {}", e))?;
                    if s.trim().is_empty() { continue; }
                    let mut prof: Profile = serde_json::from_str(&s).map_err(|e| format!("Parse: {}", e))?;
                    prof.is_default = def_name == prof.name;
                    result.push(prof);
                }
            }
        }
    }
    Ok(result)
}

pub async fn delete_profile(name: String) -> Result<(), String> {
    let dir = profiles_dir()?;
    let path = dir.join(format!("{}.json", name));
    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Delete: {}", e))?;
    }
    // If deleting default, clear default marker
    if let Ok(Some(def)) = read_default_name() {
        if def == name {
            let def_path = default_profile_path()?;
            if def_path.exists() {
                fs::remove_file(def_path).ok();
            }
        }
    }
    Ok(())
}

pub async fn set_default_profile(name: String) -> Result<(), String> {
    // Ensure profile exists
    let dir = profiles_dir()?;
    let path = dir.join(format!("{}.json", name));
    if !path.exists() {
        return Err("Profile not found".to_string());
    }
    let mut f = File::create(default_profile_path()?).map_err(|e| format!("Default write: {}", e))?;
    f.write_all(name.as_bytes()).map_err(|e| format!("Default write: {}", e))?;
    Ok(())
}
