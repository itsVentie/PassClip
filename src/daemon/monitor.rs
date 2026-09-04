use crate::config::AppConfig;
use crate::crypto::{calculate_entropy, SecureVault};
use crate::rules::RulesConfig;
use arboard::Clipboard;
use log::{error, info, trace, warn};
use notify_rust::Notification;
use regex::RegexSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::Mutex;

pub static MONITOR_PAUSED: AtomicBool = AtomicBool::new(false);

pub fn run_monitor(vault: Arc<Mutex<SecureVault>>) {
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            error!("Failed to initialize Clipboard API: {}", e);
            return;
        }
    };

    let mut last_content = String::new();
    let mut last_error: Option<String> = None;
    let config = AppConfig::load();
    let rules_config = RulesConfig::load();

    let patterns = rules_config.compile_patterns();
    let regex_set = RegexSet::new(&patterns).unwrap_or_else(|e| {
        error!("Failed to compile regex patterns: {}", e);
        RegexSet::empty()
    });

    info!(
        "Clipboard monitor thread initialized with {} active rules (version {}).",
        rules_config.rules.len(),
        rules_config.version
    );

    loop {
        sleep(Duration::from_millis(500));

        if MONITOR_PAUSED.load(Ordering::Relaxed) {
            continue;
        }

        match clipboard.get_text() {
            Ok(current_content) => {
                last_error = None;

                if current_content.is_empty() || current_content == last_content {
                    continue;
                }

                last_content = current_content.clone();

                let entropy = calculate_entropy(&current_content);
                let len = current_content.len();
                let has_space = current_content.contains(' ');
                let matches_regex = regex_set.is_match(&current_content);

                trace!(
                    "Detected clipboard change. Len: {}, Entropy: {:.2}, RegexMatch: {}, Has Space: {}",
                    len,
                    entropy,
                    matches_regex,
                    has_space
                );

                let is_sensitive = matches_regex
                    || (entropy > config.min_entropy && len >= config.min_length && !has_space);

                if is_sensitive {
                    info!("Sensitive payload detected (entropy/regex). Securing...");

                    match vault.try_lock() {
                        Ok(mut v) => {
                            let _slot_id =
                                v.multi_vault.push(current_content.clone(), entropy as f32);

                            if clipboard.set_text("").is_ok() {
                                last_content.clear();
                                info!("Clipboard wiped. Data secured in vault.");

                                if config.enable_notifications {
                                    let _ = Notification::new()
                                        .summary("PassClip Security")
                                        .body("Sensitive secret intercepted and secured.")
                                        .timeout(Duration::from_secs(3))
                                        .show();
                                }
                            } else {
                                error!("Failed to wipe clipboard contents");
                            }
                        }
                        Err(_) => {
                            warn!("Vault is currently locked by another thread");
                        }
                    }
                }
            }
            Err(e) => {
                let err_msg = e.to_string();
                if last_error.as_deref() != Some(&err_msg) {
                    trace!("Clipboard read state changed: {}", err_msg);
                    last_error = Some(err_msg);
                }
            }
        }
    }
}