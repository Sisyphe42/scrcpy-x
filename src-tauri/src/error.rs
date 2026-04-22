//! Error types for ScrcpyX

use serde::{Deserialize, Serialize};

/// Main error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScrcpyError {
    /// Binary not found (ADB or scrcpy)
    BinaryNotFound { name: String },
    /// Device not found
    DeviceNotFound { id: String },
    /// Session error
    SessionError { message: String },
    /// Profile error
    ProfileError { message: String },
    /// Settings error
    SettingsError { message: String },
    /// IO error
    IoError { message: String },
    /// Generic error
    Generic { message: String },
}

impl std::fmt::Display for ScrcpyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrcpyError::BinaryNotFound { name } => {
                write!(f, "{} not found. Please ensure it is installed and in PATH.", name)
            }
            ScrcpyError::DeviceNotFound { id } => {
                write!(f, "Device '{}' not found", id)
            }
            ScrcpyError::SessionError { message } => {
                write!(f, "Session error: {}", message)
            }
            ScrcpyError::ProfileError { message } => {
                write!(f, "Profile error: {}", message)
            }
            ScrcpyError::SettingsError { message } => {
                write!(f, "Settings error: {}", message)
            }
            ScrcpyError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            ScrcpyError::Generic { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for ScrcpyError {}
