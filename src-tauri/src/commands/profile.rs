//! Profile management commands
//!
//! Commands for managing configuration profiles.

use serde::{Deserialize, Serialize};

// Re-export storage profile type for FFI boundary
// The actual storage logic lives in the storage module.
use crate::storage::Profile as StorageProfile;

/// Get all saved profiles
#[tauri::command]
pub async fn get_profiles() -> Result<Vec<StorageProfile>, String> {
    crate::storage::list_profiles().await
}

/// Get profile by name
#[tauri::command]
pub async fn get_profile(name: String) -> Result<Option<StorageProfile>, String> {
    match crate::storage::load_profile(name).await {
        Ok(p) => Ok(Some(p)),
        Err(_) => Ok(None),
    }
}

/// Save a profile
#[tauri::command]
pub async fn save_profile(profile: StorageProfile) -> Result<(), String> {
    crate::storage::save_profile(profile).await
}

/// Delete a profile
#[tauri::command]
pub async fn delete_profile(name: String) -> Result<(), String> {
    crate::storage::delete_profile(name).await
}

/// Set default profile
#[tauri::command]
pub async fn set_default_profile(name: String) -> Result<(), String> {
    crate::storage::set_default_profile(name).await
}
