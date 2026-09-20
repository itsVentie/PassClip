use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

pub struct LogState {
    pub logs: Mutex<Vec<LogEntry>>,
}

impl LogState {
    pub fn new() -> Self {
        Self {
            logs: Mutex::new(Vec::new()),
        }
    }
}

pub fn log(app: &AppHandle, level: &str, message: &str) {
    let entry = LogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        level: level.to_string(),
        message: message.to_string(),
    };

    if let Some(state) = app.try_state::<LogState>() {
        if let Ok(mut logs) = state.logs.lock() {
            if logs.len() >= 200 {
                logs.remove(0);
            }
            logs.push(entry.clone());
        }
    }

    let _ = app.emit("daemon-log", entry);
}

#[tauri::command]
pub fn get_logs(state: State<'_, LogState>) -> Vec<LogEntry> {
    state.logs.lock().map(|l| l.clone()).unwrap_or_default()
}

#[tauri::command]
pub fn clear_logs(state: State<'_, LogState>) {
    if let Ok(mut logs) = state.logs.lock() {
        logs.clear();
    }
}