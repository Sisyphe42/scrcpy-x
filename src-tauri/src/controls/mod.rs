//! Device control commands (ADB-backed)
//!
//! This module provides high-level device control actions that are executed
//! via ADB. The public API is consumed by the Tauri command wrappers in the
//! `commands` layer.
use std::path::PathBuf;
use std::process::Command;

use crate::binaries::find_adb;

/// Send a key event to a device via ADB
pub async fn send_key_event(device_id: String, keycode: i32) -> Result<(), String> {
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;
    let status = Command::new(&adb_path)
        .args(["-s", &device_id, "shell", "input", "keyevent", &keycode.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;

    if status.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&status.stderr);
        Err(format!("ADB keyevent failed: {}", err))
    }
}

/// Take a screenshot from the device and save to a temporary file
pub async fn take_screenshot(device_id: String) -> Result<String, String> {
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;
    let status = Command::new(&adb_path)
        .args(["-s", &device_id, "shell", "screencap", "-p"])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;

    if !status.status.success() {
        let err = String::from_utf8_lossy(&status.stderr);
        return Err(format!("ADB screencap failed: {}", err));
    }

    // Persist PNG data to a temp file for the caller to consume
    let tmp_dir = std::env::temp_dir();
    let file_name = format!("scrcpyx-{}-screenshot.png", device_id);
    let mut path = PathBuf::from(&tmp_dir);
    path.push(file_name);

    std::fs::write(&path, status.stdout).map_err(|e| format!("Failed to write screenshot: {}", e))?;

    Ok(path.to_string_lossy().to_string())
}

/// Set device rotation (orientation in 0..=3)
pub async fn set_rotation(device_id: String, orientation: i32) -> Result<(), String> {
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;

    // Disable accelerometer rotation and force the requested rotation
    let cmds = [
        ["-s", &device_id, "shell", "settings", "put", "system", "accelerometer_rotation", "0"],
        ["-s", &device_id, "shell", "settings", "put", "system", "user_rotation", &orientation.to_string()],
    ];

    for c in &cmds {
        let status = Command::new(&adb_path)
            .args(c.iter().map(|s| s.to_string()).collect::<Vec<String>>())
            .output()
            .map_err(|e| format!("Failed to execute adb: {}", e))?;
        if !status.status.success() {
            let err = String::from_utf8_lossy(&status.stderr);
            return Err(format!("ADB rotation failed: {}", err));
        }
    }
    Ok(())
}

/// Set media volume for a device (volume level 0-15 typically)
pub async fn set_volume(device_id: String, level: i32) -> Result<(), String> {
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;
    let status = Command::new(&adb_path)
        .args([
            "-s",
            &device_id,
            "shell",
            "media",
            "volume",
            "--set",
            &level.to_string(),
            "--stream",
            "3",
        ])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;

    if status.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&status.stderr);
        Err(format!("ADB set volume failed: {}", err))
    }
}

/// Turn screen on by sending power key Event
pub async fn turn_screen_on(device_id: String) -> Result<(), String> {
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;
    let status = Command::new(&adb_path)
        .args(["-s", &device_id, "shell", "input", "keyevent", "26"])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;
    if status.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&status.stderr);
        Err(format!("ADB turn screen on failed: {}", err))
    }
}

/// Turn screen off by sending power key Event
pub async fn turn_screen_off(device_id: String) -> Result<(), String> {
    // Use same power key event to toggle power state
    let adb_path = find_adb().map_err(|e| format!("{}", e))?;
    let status = Command::new(&adb_path)
        .args(["-s", &device_id, "shell", "input", "keyevent", "26"])
        .output()
        .map_err(|e| format!("Failed to execute adb: {}", e))?;
    if status.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&status.stderr);
        Err(format!("ADB turn screen off failed: {}", err))
    }
}
