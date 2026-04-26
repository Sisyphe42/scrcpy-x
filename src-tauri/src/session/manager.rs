//! Session manager
//! 
//! Manages scrcpy session lifecycle: spawn, monitor, terminate.

use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::options::SessionOptions;
use crate::binaries::find_scrcpy;

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Starting,
    Running,
    Stopped,
    Error,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Device ID
    pub device_id: String,
    /// Session status
    pub status: SessionStatus,
    /// Session options
    pub options: SessionOptions,
    /// Error message (if any)
    pub error: Option<String>,
    /// Timestamp when the session was started (epoch millis)
    pub started_at: Option<i64>,
}

/// Active session with process handle
struct ActiveSession {
    session: Session,
    process: Option<Child>,
}

/// Session manager state
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, ActiveSession>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Launch a new scrcpy session
    pub async fn launch(
        &self,
        device_id: String,
        options: SessionOptions,
        app_handle: Option<tauri::AppHandle>,
    ) -> Result<Session, String> {
        let session_id = Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now().timestamp_millis();
        
        // Create session
        let session = Session {
            id: session_id.clone(),
            device_id: device_id.clone(),
            status: SessionStatus::Starting,
            options: options.clone(),
            error: None,
            started_at: Some(started_at),
        };
        
        // Find scrcpy binary
        let scrcpy_path = find_scrcpy()?;
        
        // Build command arguments
        let mut args = vec!["-s".to_string(), device_id.clone()];
        args.extend(options.to_args());
        
        // Spawn process
        let process = Command::new(&scrcpy_path)
            .args(&args)
            .spawn()
            .map_err(|e| format!("Failed to spawn scrcpy: {}", e))?;
        
        // Store session
        let active_session = ActiveSession {
            session: Session {
                status: SessionStatus::Running,
                ..session.clone()
            },
            process: Some(process),
        };
        
        let mut sessions = self.sessions.lock().await;
        sessions.insert(session_id.clone(), active_session);

        // Emit session-started event
        if let Some(ref ah) = app_handle {
            let _ = crate::events::emit_session_started(ah, &crate::events::SessionStartedPayload {
                session_id: session_id.clone(),
                device_id: device_id.clone(),
            });
        }

        // Spawn background monitor for process exit
        let monitor_sessions = self.sessions.clone();
        let monitor_session_id = session_id.clone();
        let monitor_app_handle = app_handle.clone();
        std::thread::spawn(move || {
            // Wait a moment for the session to be fully stored
            std::thread::sleep(std::time::Duration::from_millis(200));
            
            // Create a small tokio runtime just for acquiring the async mutex
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build tokio runtime for process monitor");
            
            // Take the process handle out (so we don't hold the lock while waiting)
            let process = rt.block_on(async {
                let mut sessions = monitor_sessions.lock().await;
                sessions.get_mut(&monitor_session_id).and_then(|a| a.process.take())
            });
            
            let Some(mut process) = process else { return };
            
            // Block this thread waiting for the child to exit (no lock held)
            let result = process.wait();
            
            // Now re-acquire the lock to update session status
            match result {
                Ok(exit_status) => {
                    let error_msg = if !exit_status.success() {
                        Some(format!(
                            "scrcpy exited with code: {}",
                            exit_status.code().map(|c| c.to_string()).unwrap_or_else(|| "unknown".to_string())
                        ))
                    } else {
                        None
                    };
                    
                    rt.block_on(async {
                        let mut sessions = monitor_sessions.lock().await;
                        if let Some(active) = sessions.get_mut(&monitor_session_id) {
                            active.session.status = if exit_status.success() { SessionStatus::Stopped } else { SessionStatus::Error };
                            active.session.error = error_msg.clone();
                        }
                    });

                    // Emit event to frontend
                    if let Some(ref ah) = monitor_app_handle {
                        if exit_status.success() {
                            let _ = crate::events::emit_session_ended(ah, &crate::events::SessionEndedPayload {
                                session_id: monitor_session_id.clone(),
                                reason: "Process exited normally".to_string(),
                            });
                        } else {
                            let _ = crate::events::emit_session_error(ah, &crate::events::SessionErrorPayload {
                                session_id: monitor_session_id.clone(),
                                error: error_msg.unwrap_or_else(|| "Process exited with error".to_string()),
                            });
                        }
                    }
                }
                Err(e) => {
                    rt.block_on(async {
                        let mut sessions = monitor_sessions.lock().await;
                        if let Some(active) = sessions.get_mut(&monitor_session_id) {
                            active.session.status = SessionStatus::Error;
                            active.session.error = Some(format!("Failed to wait on process: {}", e));
                        }
                    });
                    if let Some(ref ah) = monitor_app_handle {
                        let _ = crate::events::emit_session_error(ah, &crate::events::SessionErrorPayload {
                            session_id: monitor_session_id.clone(),
                            error: format!("Process monitor error: {}", e),
                        });
                    }
                }
            }
        });
        
        Ok(Session {
            status: SessionStatus::Running,
            ..session
        })
    }
    
    /// Stop a session
    pub async fn stop(&self, session_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().await;
        
        if let Some(mut active) = sessions.remove(session_id) {
            if let Some(ref mut process) = active.process {
                // Kill the process (cross-platform)
                process.kill().ok();
            }
            Ok(())
        } else {
            Err(format!("Session {} not found", session_id))
        }
    }
    
    /// Get all sessions
    pub async fn get_all(&self) -> Vec<Session> {
        let sessions = self.sessions.lock().await;
        sessions.values().map(|a| a.session.clone()).collect()
    }
    
    /// Get session by ID
    pub async fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.get(session_id).map(|a| a.session.clone())
    }
    
    /// Stop all sessions
    pub async fn stop_all(&self) {
        let mut sessions = self.sessions.lock().await;
        
        for (_, mut active) in sessions.drain() {
            if let Some(ref mut process) = active.process {
                process.kill().ok();
            }
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global session manager
lazy_static::lazy_static! {
    static ref SESSION_MANAGER: SessionManager = SessionManager::new();
}

/// Launch a new session
pub async fn launch_session(
    device_id: String,
    options: SessionOptions,
    app_handle: Option<tauri::AppHandle>,
) -> Result<Session, String> {
    SESSION_MANAGER.launch(device_id, options, app_handle).await
}

/// Stop a session
pub async fn stop_session(session_id: &str) -> Result<(), String> {
    SESSION_MANAGER.stop(session_id).await
}

/// Get all sessions
pub async fn get_sessions() -> Vec<Session> {
    SESSION_MANAGER.get_all().await
}

/// Get session by ID
pub async fn get_session_by_id(session_id: &str) -> Option<Session> {
    SESSION_MANAGER.get(session_id).await
}

/// Stop all sessions
pub async fn stop_all_sessions() {
    SESSION_MANAGER.stop_all().await
}
