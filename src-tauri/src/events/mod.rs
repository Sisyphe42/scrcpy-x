use serde::Serialize;
use tauri::Emitter;

/// Typed event names used to communicate from Rust to the frontend via Tauri.
/// All events are emitted using the AppHandle.emit(...) method and payloads
/// derive Serialize for proper interop with the frontend.

/// Device connected event payload
/// Payload: { device_id: String, name: String }
#[derive(Clone, Serialize)]
pub struct DeviceConnectedPayload {
    pub device_id: String,
    pub name: String,
}

/// Device disconnected event payload
/// Payload: { device_id: String }
#[derive(Clone, Serialize)]
pub struct DeviceDisconnectedPayload {
    pub device_id: String,
}

/// Session started event payload
/// Payload: { session_id: String, device_id: String }
#[derive(Clone, Serialize)]
pub struct SessionStartedPayload {
    pub session_id: String,
    pub device_id: String,
}

/// Session ended event payload
/// Payload: { session_id: String, reason: String }
#[derive(Clone, Serialize)]
pub struct SessionEndedPayload {
    pub session_id: String,
    pub reason: String,
}

/// Session error event payload
/// Payload: { session_id: String, error: String }
#[derive(Clone, Serialize)]
pub struct SessionErrorPayload {
    pub session_id: String,
    pub error: String,
}

// Event name constants to avoid typos in string literals
pub const DEVICE_CONNECTED: &str = "device-connected";
pub const DEVICE_DISCONNECTED: &str = "device-disconnected";
pub const SESSION_STARTED: &str = "session-started";
pub const SESSION_ENDED: &str = "session-ended";
pub const SESSION_ERROR: &str = "session-error";

/// Generic event emitter helper. Can be used by any task/component that has access
/// to the Tauri AppHandle and wants to emit a structured payload for a given event.
///
/// This function is intentionally small and generic to avoid duplicating emission logic
/// across different event wrappers.
pub fn emit_event<T: Serialize>(app_handle: &tauri::AppHandle, event: &str, payload: &T) -> tauri::Result<()> {
    app_handle.emit(event, payload)
}

/// Emit device-connected event
pub fn emit_device_connected(app_handle: &tauri::AppHandle, payload: &DeviceConnectedPayload) -> tauri::Result<()> {
    emit_event(app_handle, DEVICE_CONNECTED, payload)
}

/// Emit device-disconnected event
pub fn emit_device_disconnected(app_handle: &tauri::AppHandle, payload: &DeviceDisconnectedPayload) -> tauri::Result<()> {
    emit_event(app_handle, DEVICE_DISCONNECTED, payload)
}

/// Emit session-started event
pub fn emit_session_started(app_handle: &tauri::AppHandle, payload: &SessionStartedPayload) -> tauri::Result<()> {
    emit_event(app_handle, SESSION_STARTED, payload)
}

/// Emit session-ended event
pub fn emit_session_ended(app_handle: &tauri::AppHandle, payload: &SessionEndedPayload) -> tauri::Result<()> {
    emit_event(app_handle, SESSION_ENDED, payload)
}

/// Emit session-error event
pub fn emit_session_error(app_handle: &tauri::AppHandle, payload: &SessionErrorPayload) -> tauri::Result<()> {
    emit_event(app_handle, SESSION_ERROR, payload)
}
