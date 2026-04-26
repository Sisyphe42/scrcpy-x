//! Window management commands
//!
//! Commands for creating and managing auxiliary Tauri windows
//! (e.g., floating device control panel).

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// Open the floating device control panel window.
/// If it already exists, focus it instead of creating a duplicate.
#[tauri::command]
pub async fn open_floating_panel(app: tauri::AppHandle) -> Result<(), String> {
    let label = "floating-panel";

    // If the window already exists, just focus it
    if let Some(existing) = app.get_webview_window(label) {
        existing
            .set_focus()
            .map_err(|e| format!("Failed to focus floating panel: {}", e))?;
        return Ok(());
    }

    // Create a new floating always-on-top window
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("/floating-panel".into()))
        .title("ScrcpyX — Controls")
        .inner_size(360.0, 480.0)
        .min_inner_size(280.0, 360.0)
        .always_on_top(true)
        .decorations(true)
        .resizable(true)
        .skip_taskbar(false)
        .build()
        .map_err(|e| format!("Failed to create floating panel: {}", e))?;

    Ok(())
}

/// Close the floating device control panel window.
#[tauri::command]
pub async fn close_floating_panel(app: tauri::AppHandle) -> Result<(), String> {
    let label = "floating-panel";
    if let Some(window) = app.get_webview_window(label) {
        window
            .close()
            .map_err(|e| format!("Failed to close floating panel: {}", e))?;
    }
    Ok(())
}
