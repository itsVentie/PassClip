mod autostart;
mod commands;
mod logs;
mod tray;

use commands::{get_vault_slots, get_vault_status, pop_slot, verify_assertion};
use logs::{clear_logs, get_logs, log, LogState};
use tauri::WindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(LogState::new())
        .setup(|app| {
            tray::setup_tray(app.handle())?;

            let handle = app.handle().clone();
            log(&handle, "INFO", "PassClip GUI initialized");

            tauri::async_runtime::spawn(async move {
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
            verify_assertion,
            get_logs,
            clear_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}