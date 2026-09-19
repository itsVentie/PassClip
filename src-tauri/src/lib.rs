mod autostart;
mod commands;
mod tray;

use tauri::WindowEvent;
use commands::{get_vault_slots, get_vault_status, pop_slot, verify_assertion};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                autostart::ensure_daemon_running().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_vault_slots,
            get_vault_status,
            pop_slot,
            verify_assertion
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}