//! Session management commands
//! 
//! Commands for launching and managing scrcpy sessions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::session::{self, Session, SessionStatus, SessionOptions as SessionOpts};

/// Session options (scrcpy configuration)
/// This is a simplified version for the frontend
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionOptions {
    // Connection options
    pub serial: Option<String>,
    pub tcpip: Option<String>,
    
    // Video options
    pub video_codec: Option<String>,
    pub video_bitrate: Option<u32>,
    pub max_size: Option<u16>,
    pub max_fps: Option<u16>,
    pub crop: Option<String>,
    
    // Audio options
    pub audio: Option<bool>,
    pub audio_codec: Option<String>,
    pub audio_bitrate: Option<u32>,
    pub audio_source: Option<String>,
    
    // Recording options
    pub record: Option<String>,
    pub record_format: Option<String>,
    
    // Window options
    pub fullscreen: Option<bool>,
    pub always_on_top: Option<bool>,
    pub borderless: Option<bool>,
    pub window_title: Option<String>,
    pub window_x: Option<i16>,
    pub window_y: Option<i16>,
    pub window_width: Option<u16>,
    pub window_height: Option<u16>,
    
    // Device behavior options
    pub turn_screen_off: Option<bool>,
    pub stay_awake: Option<bool>,
    pub show_touches: Option<bool>,
    
    // Input control options
    pub keyboard: Option<String>,
    pub mouse: Option<String>,
    pub gamepad: Option<String>,
    pub shortcut_mod: Option<String>,
    
    // Additional options
    pub extra: Option<HashMap<String, String>>,
}

impl From<SessionOptions> for SessionOpts {
    fn from(opts: SessionOptions) -> Self {
        SessionOpts {
            serial: opts.serial,
            tcpip: opts.tcpip,
            port_range: None,
            video_codec: opts.video_codec,
            video_bitrate: opts.video_bitrate,
            max_size: opts.max_size,
            max_fps: opts.max_fps,
            crop: opts.crop,
            audio: opts.audio,
            audio_codec: opts.audio_codec,
            audio_bitrate: opts.audio_bitrate,
            audio_source: opts.audio_source,
            record: opts.record,
            record_format: opts.record_format,
            fullscreen: opts.fullscreen,
            always_on_top: opts.always_on_top,
            borderless: opts.borderless,
            window_title: opts.window_title,
            window_x: opts.window_x,
            window_y: opts.window_y,
            window_width: opts.window_width,
            window_height: opts.window_height,
            turn_screen_off: opts.turn_screen_off,
            stay_awake: opts.stay_awake,
            show_touches: opts.show_touches,
            keyboard: opts.keyboard,
            mouse: opts.mouse,
            gamepad: opts.gamepad,
            shortcut_mod: opts.shortcut_mod,
            extra: opts.extra,
        }
    }
}

/// Launch a new scrcpy session
#[tauri::command]
pub async fn launch_session(
    device_id: String,
    options: SessionOptions,
) -> Result<Session, String> {
    let opts: SessionOpts = options.into();
    session::launch_session(device_id, opts).await
}

/// Stop a running session
#[tauri::command]
pub async fn stop_session(session_id: String) -> Result<(), String> {
    session::stop_session(&session_id).await
}

/// Get all active sessions
#[tauri::command]
pub async fn get_sessions() -> Result<Vec<Session>, String> {
    Ok(session::get_sessions().await)
}

/// Get session by ID
#[tauri::command]
pub async fn get_session(session_id: String) -> Result<Option<Session>, String> {
    Ok(session::get_session_by_id(&session_id).await)
}
