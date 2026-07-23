use crate::config::AppConfig;
use crate::crypto::{calculate_entropy, SecureVault};
use arboard::Clipboard;
use log::{debug, error, info, warn};
use notify_rust::Notification;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::Mutex;

pub fn run_monitor(vault: Arc<Mutex<SecureVault>>) {
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            error!("Failed to initialize Clipboard API: {}", e);
            return;
        }
    };

    let mut last_content = String::new();
    let config = AppConfig::load();
    debug!("Clipboard monitor thread successfully initialized.");

    loop {
        sleep(Duration::from_millis(500));

        match clipboard.get_text() {
            Ok(current_content) => {
                if current_content.is_empty() || current_content == last_content {
                    continue;
                }

                last_content = current_content.clone();

                let entropy = calculate_entropy(&current_content);
                let len = current_content.len();
                let has_space = current_content.contains(' ');

                debug!(
                    "Detected change. Len: {}, Entropy: {:.2}, Has Space: {}",
                    len, entropy, has_space
                );

                if entropy > config.min_entropy && len >= config.min_length && !has_space {
                    info!("Criteria matched. Securing sensitive payload...");

                    match vault.try_lock() {
                        Ok(mut v) => {
                            if v.protect(&current_content).is_ok() {
                                if clipboard.set_text("").is_ok() {
                                    last_content.clear();
                                    info!("Clipboard wiped. Data secured in vault.");

                                    let _ = Notification::new()
                                        .summary("PassClip Security")
                                        .body("High-entropy secret intercepted and secured.")
                                        .timeout(Duration::from_secs(3))
                                        .show();
                                } else {
                                    error!("Failed to wipe clipboard contents");
                                }
                            } else {
                                error!("Encryption failed inside vault");
                            }
                        }
                        Err(_) => {
                            warn!("Vault is currently locked by another thread");
                        }
                    }
                } else {
                    debug!("Ignored: payload does not match security criteria.");
                }
            }
            Err(e) => {
                debug!("Clipboard read state: {}", e);
            }
        }
    }
}