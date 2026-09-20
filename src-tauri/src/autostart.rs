use crate::logs;
use passclip::ipc::protocol::IpcRequest;
use passclip::ipc::server::send_client_request;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::AppHandle;

pub async fn ensure_daemon_running(app: &AppHandle) {
    if send_client_request(IpcRequest::GetStatus).await.is_ok() {
        logs::log(app, "INFO", "Daemon process is already running");
        return;
    }

    logs::log(app, "WARN", "Daemon IPC unreachable. Attempting auto-spawn...");

    #[cfg(debug_assertions)]
    let mut cmd = Command::new("cargo");
    #[cfg(debug_assertions)]
    cmd.args(["run", "--package", "passclip", "--bin", "passclip", "--", "daemon"]);

    #[cfg(not(debug_assertions))]
    let mut cmd = Command::new("passclip");
    #[cfg(not(debug_assertions))]
    cmd.arg("daemon");

    match cmd.spawn() {
        Ok(child) => {
            logs::log(
                app,
                "INFO",
                &format!("Daemon process spawned successfully (PID: {})", child.id()),
            );
            thread::sleep(Duration::from_millis(500));
        }
        Err(e) => {
            logs::log(
                app,
                "ERROR",
                &format!("Failed to spawn daemon process: {}", e),
            );
        }
    }
}