use crate::crypto::SecureVault;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

pub fn start_zeroize_worker(vault: Arc<Mutex<SecureVault>>) {
    let timeout_secs = 600; 
    let max_age = Duration::from_secs(timeout_secs);
    let check_interval = Duration::from_secs((timeout_secs / 2).max(1));

    log::info!(
        "Zeroize background worker initialized (timeout: {}s, check interval: {:?})",
        timeout_secs,
        check_interval
    );

    tokio::spawn(async move {
        loop {
            sleep(check_interval).await;
            let mut guard = vault.lock().await;
            guard.cleanup_expired(max_age);
        }
    });
}