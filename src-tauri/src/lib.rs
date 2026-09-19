mod autostart;
mod commands;
mod tray;

use tauri::WindowEvent;
use commands::{get_vault_slots, get_vault_status, pop_slot, verify_assertion};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tray::setup_tray(app.handle())?;

            tauri::async_runtime::spawn(async {
                autostart::ensure_daemon_running().await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
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