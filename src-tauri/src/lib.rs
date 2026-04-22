// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod binaries;
pub mod commands;
pub mod controls;
pub mod device;
pub mod error;
pub mod session;
pub mod storage;
pub mod settings;
pub mod events;

use commands::{
    get_devices, refresh_devices,
    launch_session, stop_session, get_sessions, get_session,
    get_profiles, get_profile, save_profile, delete_profile, set_default_profile,
    get_settings, save_settings, get_adb_path, get_scrcpy_path,
    send_key_event, take_screenshot, set_rotation, set_volume, turn_screen_on, turn_screen_off,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Attempt to preload settings on startup (best-effort)
    let _ = crate::settings::load_settings();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| {
            // Auto-save settings on close request
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if let Ok(s) = crate::settings::load_settings() {
                    let _ = crate::settings::save_settings(s);
                }
            }
            // Allow other events to propagate
            let _ = window;
        })
        .invoke_handler(tauri::generate_handler![
            // Device commands
            get_devices,
            refresh_devices,
            // Session commands
            launch_session,
            stop_session,
            get_sessions,
            get_session,
            // Profile commands
            get_profiles,
            get_profile,
            save_profile,
            delete_profile,
            set_default_profile,
            // Settings commands
            get_settings,
            save_settings,
            get_adb_path,
            get_scrcpy_path,
            // Controls commands
            send_key_event,
            take_screenshot,
            set_rotation,
            set_volume,
            turn_screen_on,
            turn_screen_off,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
