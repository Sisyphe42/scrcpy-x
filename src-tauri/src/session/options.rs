//! Session options
//! 
//! Defines the configuration options for scrcpy sessions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Session options (scrcpy configuration)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionOptions {
    // Connection options
    /// Device serial (if None, uses default device)
    pub serial: Option<String>,
    /// Use TCP/IP connection
    pub tcpip: Option<String>,
    /// Port range for connection
    pub port_range: Option<String>,
    
    // Video options
    /// Video codec (h264, h265, av1)
    pub video_codec: Option<String>,
    /// Video bitrate in bits/s
    pub video_bitrate: Option<u32>,
    /// Maximum video dimension
    pub max_size: Option<u16>,
    /// Maximum frame rate
    pub max_fps: Option<u16>,
    /// Crop region (width:height:x:y)
    pub crop: Option<String>,
    
    // Audio options
    /// Enable/disable audio
    pub audio: Option<bool>,
    /// Audio codec (opus, aac, flac, raw)
    pub audio_codec: Option<String>,
    /// Audio bitrate in bits/s
    pub audio_bitrate: Option<u32>,
    /// Audio source
    pub audio_source: Option<String>,
    
    // Recording options
    /// Record to file
    pub record: Option<String>,
    /// Recording format (mp4, mkv, etc.)
    pub record_format: Option<String>,
    
    // Window options
    /// Start in fullscreen
    pub fullscreen: Option<bool>,
    /// Always on top
    pub always_on_top: Option<bool>,
    /// Borderless window
    pub borderless: Option<bool>,
    /// Custom window title
    pub window_title: Option<String>,
    /// Window X position
    pub window_x: Option<i16>,
    /// Window Y position
    pub window_y: Option<i16>,
    /// Window width
    pub window_width: Option<u16>,
    /// Window height
    pub window_height: Option<u16>,
    
    // Device behavior options
    /// Turn device screen off
    pub turn_screen_off: Option<bool>,
    /// Keep device awake
    pub stay_awake: Option<bool>,
    /// Show touches
    pub show_touches: Option<bool>,
    
    // Input control options
    /// Keyboard input mode (disabled, sdk, uhid, aoa)
    pub keyboard: Option<String>,
    /// Mouse input mode (disabled, sdk, uhid, aoa)
    pub mouse: Option<String>,
    /// Gamepad input mode
    pub gamepad: Option<String>,
    /// Shortcut modifier
    pub shortcut_mod: Option<String>,
    
    // Additional options
    /// Extra CLI arguments
    pub extra: Option<HashMap<String, String>>,
}

impl SessionOptions {
    /// Convert options to scrcpy CLI arguments
    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        
        // Connection options
        if let Some(ref serial) = self.serial {
            args.push("-s".to_string());
            args.push(serial.clone());
        }
        
        if let Some(ref tcpip) = self.tcpip {
            args.push("--tcpip".to_string());
            args.push(tcpip.clone());
        }
        
        // Video options
        if let Some(ref codec) = self.video_codec {
            args.push(format!("--video-codec={}", codec));
        }
        
        if let Some(bitrate) = self.video_bitrate {
            args.push(format!("--video-bit-rate={}", bitrate));
        }
        
        if let Some(size) = self.max_size {
            args.push(format!("--max-size={}", size));
        }
        
        if let Some(fps) = self.max_fps {
            args.push(format!("--max-fps={}", fps));
        }
        
        if let Some(ref crop) = self.crop {
            args.push(format!("--crop={}", crop));
        }
        
        // Audio options
        if let Some(audio) = self.audio {
            if !audio {
                args.push("--no-audio".to_string());
            }
        }
        
        if let Some(ref codec) = self.audio_codec {
            args.push(format!("--audio-codec={}", codec));
        }
        
        if let Some(bitrate) = self.audio_bitrate {
            args.push(format!("--audio-bit-rate={}", bitrate));
        }
        
        // Recording options
        if let Some(ref record) = self.record {
            args.push(format!("--record={}", record));
        }
        
        if let Some(ref format) = self.record_format {
            args.push(format!("--record-format={}", format));
        }
        
        // Window options
        if let Some(true) = self.fullscreen {
            args.push("-f".to_string());
        }
        
        if let Some(true) = self.always_on_top {
            args.push("--always-on-top".to_string());
        }
        
        if let Some(true) = self.borderless {
            args.push("--window-borderless".to_string());
        }
        
        if let Some(ref title) = self.window_title {
            args.push(format!("--window-title={}", title));
        }
        
        // Device behavior
        if let Some(true) = self.turn_screen_off {
            args.push("-S".to_string());
        }
        
        if let Some(true) = self.stay_awake {
            args.push("-w".to_string());
        }
        
        if let Some(true) = self.show_touches {
            args.push("-t".to_string());
        }
        
        // Input control
        if let Some(ref keyboard) = self.keyboard {
            args.push(format!("--keyboard={}", keyboard));
        }
        
        if let Some(ref mouse) = self.mouse {
            args.push(format!("--mouse={}", mouse));
        }
        
        args
    }
}
