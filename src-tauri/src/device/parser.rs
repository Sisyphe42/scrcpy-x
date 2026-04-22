//! ADB output parser
//! 
//! Parses the output of `adb devices -l` to extract device information.

use serde::{Deserialize, Serialize};

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Device ID (serial number)
    pub id: String,
    /// Device name/model
    pub name: String,
    /// Device model
    pub model: String,
    /// Device status (online, offline, unauthorized)
    pub status: DeviceStatus,
}

/// Device connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Online,
    Offline,
    Unauthorized,
}

impl std::fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceStatus::Online => write!(f, "online"),
            DeviceStatus::Offline => write!(f, "offline"),
            DeviceStatus::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

/// Parse `adb devices -l` output
/// 
/// Example output:
/// ```
/// List of devices attached
/// emulator-5554          device product:sdk_gphone64_x86_64 model:sdk_gphone64_x86_64 device:emu64x16
/// abc123                  unauthorized
/// ```
pub fn parse_adb_devices(output: &str) -> Vec<Device> {
    let mut devices = Vec::new();
    
    for line in output.lines() {
        // Skip header line
        if line.starts_with("List of devices") || line.is_empty() {
            continue;
        }
        
        // Parse device line
        if let Some(device) = parse_device_line(line) {
            devices.push(device);
        }
    }
    
    devices
}

/// Parse a single device line from `adb devices -l` output
fn parse_device_line(line: &str) -> Option<Device> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return None;
    }
    
    let id = parts[0].to_string();
    let status_str = parts[1];
    
    let status = match status_str {
        "device" => DeviceStatus::Online,
        "offline" => DeviceStatus::Offline,
        "unauthorized" => DeviceStatus::Unauthorized,
        _ => return None,
    };
    
    // Extract model from the rest of the line
    let model = extract_model(&parts[2..]);
    let name = model.clone();
    
    Some(Device {
        id,
        name,
        model,
        status,
    })
}

/// Extract model from device info parts
fn extract_model(parts: &[&str]) -> String {
    for part in parts {
        if part.starts_with("model:") {
            return part.trim_start_matches("model:").to_string();
        }
    }
    "Unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_adb_devices() {
        let output = r#"List of devices attached
emulator-5554          device product:sdk_gphone64_x86_64 model:sdk_gphone64_x86_64 device:emu64x16
abc123                  unauthorized
"#;
        let devices = parse_adb_devices(output);
        
        assert_eq!(devices.len(), 2);
        
        assert_eq!(devices[0].id, "emulator-5554");
        assert_eq!(devices[0].status, DeviceStatus::Online);
        assert_eq!(devices[0].model, "sdk_gphone64_x86_64");
        
        assert_eq!(devices[1].id, "abc123");
        assert_eq!(devices[1].status, DeviceStatus::Unauthorized);
    }
    
    #[test]
    fn test_parse_empty_devices() {
        let output = "List of devices attached\n";
        let devices = parse_adb_devices(output);
        assert!(devices.is_empty());
    }
}
