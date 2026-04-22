//! Device discovery via ADB
//! 
//! Implements device discovery by running `adb devices -l` and parsing the output.

use std::process::Command;
use crate::binaries::find_adb;
use super::parser::parse_adb_devices;
use super::{Device, DeviceStatus};

/// Discover connected devices via ADB
pub async fn discover_devices() -> Result<Vec<Device>, String> {
    // Find ADB binary
    let adb_path = find_adb()?;
    
    // Run `adb devices -l`
    let output = Command::new(&adb_path)
        .args(["devices", "-l"])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ADB command failed: {}", stderr));
    }
    
    // Parse the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let devices = parse_adb_devices(&stdout);
    
    Ok(devices)
}

/// Check if ADB server is running
pub async fn is_adb_server_running() -> bool {
    if let Ok(adb_path) = find_adb() {
        if let Ok(output) = Command::new(&adb_path)
            .args(["devices"])
            .output()
        {
            return output.status.success();
        }
    }
    false
}

/// Get detailed device info
pub async fn get_device_info(device_id: &str) -> Result<Device, String> {
    let adb_path = find_adb()?;
    
    // Get device model
    let model_output = Command::new(&adb_path)
        .args(["-s", device_id, "shell", "getprop", "ro.product.model"])
        .output()
        .map_err(|e| format!("Failed to get device model: {}", e))?;
    
    let model = String::from_utf8_lossy(&model_output.stdout)
        .trim()
        .to_string();
    
    // Get device name
    let name_output = Command::new(&adb_path)
        .args(["-s", device_id, "shell", "getprop", "ro.product.name"])
        .output()
        .map_err(|e| format!("Failed to get device name: {}", e))?;
    
    let name = String::from_utf8_lossy(&name_output.stdout)
        .trim()
        .to_string();
    
    Ok(Device {
        id: device_id.to_string(),
        name: if name.is_empty() { model.clone() } else { name },
        model,
        status: DeviceStatus::Online,
    })
}

/// Kill ADB server
pub async fn kill_adb_server() -> Result<(), String> {
    let adb_path = find_adb()?;
    
    Command::new(&adb_path)
        .args(["kill-server"])
        .output()
        .map_err(|e| format!("Failed to kill ADB server: {}", e))?;
    
    Ok(())
}

/// Start ADB server
pub async fn start_adb_server() -> Result<(), String> {
    let adb_path = find_adb()?;
    
    Command::new(&adb_path)
        .args(["start-server"])
        .output()
        .map_err(|e| format!("Failed to start ADB server: {}", e))?;
    
    Ok(())
}
