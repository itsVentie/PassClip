mod autostart;

use passclip::ipc::protocol::{IpcRequest, IpcResponse};
use passclip::ipc::server::send_client_request;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
pub struct UiSlot {
    pub id: u32,
    pub len: usize,
    pub entropy: f32,
    pub timestamp: u64,
}

#[tauri::command]
async fn get_vault_slots() -> Result<Vec<UiSlot>, String> {
    match send_client_request(IpcRequest::List).await {
        Ok(IpcResponse::List { slots }) => {
            let ui_slots = slots
                .into_iter()
                .map(|s| {
                    let timestamp_secs = s
                        .timestamp
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    UiSlot {
                        id: s.id,
                        len: s.len,
                        entropy: s.entropy,
                        timestamp: timestamp_secs,
                    }
                })
                .collect();
            Ok(ui_slots)
        }
        Ok(IpcResponse::Error { message }) => Err(message),
        Err(e) => Err(format!("Daemon connection failed: {}", e)),
        _ => Err("Unexpected response from daemon".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                autostart::ensure_daemon_running().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_vault_slots])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}