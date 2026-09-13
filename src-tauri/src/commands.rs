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

#[derive(Serialize, Deserialize)]
pub struct UiStatus {
    pub has_secret: bool,
    pub count: usize,
    pub max_slots: usize,
}

#[tauri::command]
pub async fn get_vault_slots() -> Result<Vec<UiSlot>, String> {
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

#[tauri::command]
pub async fn get_vault_status() -> Result<UiStatus, String> {
    match send_client_request(IpcRequest::GetStatus).await {
        Ok(IpcResponse::Status {
            has_secret,
            count,
            max_slots,
        }) => Ok(UiStatus {
            has_secret,
            count,
            max_slots,
        }),
        Ok(IpcResponse::Error { message }) => Err(message),
        Err(e) => Err(format!("Failed to connect to daemon: {}", e)),
        _ => Err("Unexpected response from daemon".into()),
    }
}

#[tauri::command]
pub async fn pop_slot(id: u32) -> Result<String, String> {
    match send_client_request(IpcRequest::RequestChallenge { slot_id: Some(id) }).await {
        Ok(IpcResponse::Challenge { options }) => {
            let payload = serde_json::to_string(&options)
                .map_err(|e| format!("Failed to serialize challenge: {}", e))?;
            Ok(payload)
        }
        Ok(IpcResponse::Error { message }) => Err(message),
        Err(e) => Err(format!("IPC error: {}", e)),
        _ => Err("Unexpected response".into()),
    }
}

#[tauri::command]
pub async fn verify_assertion(assertion_json: String) -> Result<String, String> {
    let credential = serde_json::from_str(&assertion_json)
        .map_err(|e| format!("Failed to parse credential assertion JSON: {}", e))?;

    let request = IpcRequest::VerifyAssertion {
        assertion: Box::new(credential),
    };

    match send_client_request(request).await {
        Ok(IpcResponse::Success { secret }) => Ok(secret.as_str().to_string()),
        Ok(IpcResponse::Error { message }) => Err(message),
        Err(e) => Err(format!("IPC connection error: {}", e)),
        _ => Err("Unexpected response from daemon".into()),
    }
}