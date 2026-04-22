//! Tauri command wrappers for device controls
//!
//! These commands delegate to the underlying ADB-based control module.

use crate::controls::{send_key_event as ctrl_send_key_event,
                     take_screenshot as ctrl_take_screenshot,
                     set_rotation as ctrl_set_rotation,
                     set_volume as ctrl_set_volume,
                     turn_screen_on as ctrl_turn_screen_on,
                     turn_screen_off as ctrl_turn_screen_off};

/// Send a key event to a device
#[tauri::command]
pub async fn send_key_event(device_id: String, keycode: i32) -> Result<(), String> {
    ctrl_send_key_event(device_id, keycode).await
}

/// Take a screenshot from a device and return path to saved PNG
#[tauri::command]
pub async fn take_screenshot(device_id: String) -> Result<String, String> {
    ctrl_take_screenshot(device_id).await
}

/// Set screen rotation (0-3)
#[tauri::command]
pub async fn set_rotation(device_id: String, orientation: i32) -> Result<(), String> {
    ctrl_set_rotation(device_id, orientation).await
}

/// Set media volume for a device (level 0-15 typical)
#[tauri::command]
pub async fn set_volume(device_id: String, level: i32) -> Result<(), String> {
    ctrl_set_volume(device_id, level).await
}

/// Turn screen on
#[tauri::command]
pub async fn turn_screen_on(device_id: String) -> Result<(), String> {
    ctrl_turn_screen_on(device_id).await
}

/// Turn screen off
#[tauri::command]
pub async fn turn_screen_off(device_id: String) -> Result<(), String> {
    ctrl_turn_screen_off(device_id).await
}
