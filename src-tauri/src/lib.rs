// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod binaries;
pub mod commands;
pub mod device;
pub mod error;

use commands::{
    get_devices, refresh_devices,
    launch_session, stop_session, get_sessions, get_session,
    get_profiles, get_profile, save_profile, delete_profile, set_default_profile,
    get_settings, save_settings, get_adb_path, get_scrcpy_path,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
