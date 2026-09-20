use crate::logs;
use passclip::ipc::protocol::{IpcRequest, IpcResponse};
use passclip::ipc::server::send_client_request;
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;
use tauri::AppHandle;

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
pub async fn get_vault_slots(app: AppHandle) -> Result<Vec<UiSlot>, String> {
    logs::log(&app, "DEBUG", "Fetching vault slots from daemon...");
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
        Ok(IpcResponse::Error { message }) => {
            logs::log(&app, "ERROR", &format!("Failed to fetch slots: {}", message));
            Err(message)
        }
        Err(e) => {
            let err_msg = format!("Daemon connection failed: {}", e);
            logs::log(&app, "ERROR", &err_msg);
            Err(err_msg)
        }
        _ => {
            logs::log(&app, "ERROR", "Unexpected response from daemon on get_vault_slots");
            Err("Unexpected response from daemon".into())
        }
    }
}

#[tauri::command]
pub async fn get_vault_status(app: AppHandle) -> Result<UiStatus, String> {
    logs::log(&app, "DEBUG", "Fetching vault status...");
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
        Ok(IpcResponse::Error { message }) => {
            logs::log(&app, "ERROR", &format!("Daemon status error: {}", message));
            Err(message)
        }
        Err(e) => {
            let err_msg = format!("Failed to connect to daemon: {}", e);
            logs::log(&app, "ERROR", &err_msg);
            Err(err_msg)
        }
        _ => {
            logs::log(&app, "ERROR", "Unexpected response from daemon on get_vault_status");
            Err("Unexpected response from daemon".into())
        }
    }
}

#[tauri::command]
pub async fn pop_slot(app: AppHandle, id: u32) -> Result<String, String> {
    logs::log(&app, "INFO", &format!("Requesting challenge for slot ID: {}", id));
    match send_client_request(IpcRequest::RequestChallenge { slot_id: Some(id) }).await {
        Ok(IpcResponse::Challenge { options }) => {
            let payload = serde_json::to_string(&options)
                .map_err(|e| format!("Failed to serialize challenge: {}", e))?;
            logs::log(&app, "INFO", &format!("Challenge generated for slot ID: {}", id));
            Ok(payload)
        }
        Ok(IpcResponse::Error { message }) => {
            logs::log(&app, "ERROR", &format!("Pop slot error: {}", message));
            Err(message)
        }
        Err(e) => {
            let err_msg = format!("IPC error: {}", e);
            logs::log(&app, "ERROR", &err_msg);
            Err(err_msg)
        }
        _ => {
            logs::log(&app, "ERROR", "Unexpected response from daemon on pop_slot");
            Err("Unexpected response".into())
        }
    }
}

#[tauri::command]
pub async fn verify_assertion(app: AppHandle, assertion_json: String) -> Result<String, String> {
    logs::log(&app, "INFO", "Verifying credential assertion...");
    let credential = serde_json::from_str(&assertion_json).map_err(|e| {
        let err_msg = format!("Failed to parse credential assertion JSON: {}", e);
        logs::log(&app, "ERROR", &err_msg);
        err_msg
    })?;

    let request = IpcRequest::VerifyAssertion {
        assertion: Box::new(credential),
    };

    match send_client_request(request).await {
        Ok(IpcResponse::Success { secret }) => {
            logs::log(&app, "INFO", "Assertion verified successfully. Vault secret retrieved");
            Ok(secret.as_str().to_string())
        }
        Ok(IpcResponse::Error { message }) => {
            logs::log(&app, "ERROR", &format!("Assertion verification failed: {}", message));
            Err(message)
        }
        Err(e) => {
            let err_msg = format!("IPC connection error: {}", e);
            logs::log(&app, "ERROR", &err_msg);
            Err(err_msg)
        }
        _ => {
            logs::log(&app, "ERROR", "Unexpected response from daemon on verify_assertion");
            Err("Unexpected response from daemon".into())
        }
    }
}