use log::{info, warn};
use std::process::Command;
use std::thread;
use std::time::Duration;
use passclip::ipc::protocol::IpcRequest;
use passclip::ipc::server::send_client_request;

pub async fn ensure_daemon_running() {
    if send_client_request(IpcRequest::GetStatus).await.is_ok() {
        info!("Daemon is already running.");
        return;
    }

    warn!("Daemon IPC unreachable. Attempting auto-spawn...");

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
            info!("Daemon process spawned successfully (PID: {}).", child.id());
            thread::sleep(Duration::from_millis(500));
        }
        Err(e) => warn!("Failed to spawn daemon process: {}", e),
    }
}