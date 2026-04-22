//! Device discovery commands
//! 
//! Commands for discovering and managing connected Android devices.

use serde::{Deserialize, Serialize};
use crate::device::{self, Device, DeviceStatus};

// Re-export Device and DeviceStatus for use in other modules
pub use crate::device::{Device as DeviceInfo, DeviceStatus as ConnectionStatus};

/// Get list of connected devices
#[tauri::command]
pub async fn get_devices() -> Result<Vec<Device>, String> {
    device::discover_devices().await
}

/// Refresh device list
#[tauri::command]
pub async fn refresh_devices() -> Result<Vec<Device>, String> {
    device::discover_devices().await
}
