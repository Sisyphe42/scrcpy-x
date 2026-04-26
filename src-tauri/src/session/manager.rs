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
    ) -> Result<Session, String> {
        let session_id = Uuid::new_v4().to_string();
        
        // Create session
        let session = Session {
            id: session_id.clone(),
            device_id: device_id.clone(),
            status: SessionStatus::Starting,
            options: options.clone(),
            error: None,
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
                // Try graceful termination first
                #[cfg(target_os = "windows")]
                process.kill().ok();
                
                #[cfg(not(target_os = "windows"))]
                {
                    use std::signal::Signal;
                    process.signal(Signal::SIGTERM).ok();
                }
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
) -> Result<Session, String> {
    SESSION_MANAGER.launch(device_id, options).await
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
