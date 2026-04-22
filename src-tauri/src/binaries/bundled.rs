//! Bundled binary paths
//! 
//! Provides paths to bundled ADB and scrcpy binaries.
//! These binaries are included in the app for zero-config setup.

use std::path::PathBuf;

/// Get the path to bundled ADB binary
/// 
/// Returns None if no bundled binary is available.
/// The actual binary should be placed in the resources during build.
pub fn get_bundled_adb_path() -> Option<PathBuf> {
    // In production, bundled binaries would be in the app resources
    // For now, return None as we don't have bundled binaries yet
    // TODO: Implement bundled binary path resolution based on platform
    
    #[cfg(target_os = "windows")]
    {
        // Windows: Check for bundled adb.exe in resources
        None
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Check in app bundle Resources
        None
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux: Check in /usr/lib/scrcpyx or similar
        None
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

/// Get the path to bundled scrcpy binary
/// 
/// Returns None if no bundled binary is available.
/// The actual binary should be placed in the resources during build.
pub fn get_bundled_scrcpy_path() -> Option<PathBuf> {
    // In production, bundled binaries would be in the app resources
    // For now, return None as we don't have bundled binaries yet
    // TODO: Implement bundled binary path resolution based on platform
    
    #[cfg(target_os = "windows")]
    {
        // Windows: Check for bundled scrcpy.exe in resources
        None
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: Check in app bundle Resources
        None
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux: Check in /usr/lib/scrcpyx or similar
        None
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

/// Get the path to bundled scrcpy server
/// 
/// scrcpy requires a server JAR to be pushed to the device.
pub fn get_bundled_scrcpy_server_path() -> Option<PathBuf> {
    // TODO: Implement bundled server path resolution
    None
}

/// Check if bundled binaries are available
pub fn has_bundled_binaries() -> bool {
    get_bundled_adb_path().is_some() && get_bundled_scrcpy_path().is_some()
}
