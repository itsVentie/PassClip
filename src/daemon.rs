use crate::crypto::{calculate_entropy, SecureVault};
use arboard::Clipboard;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::Mutex;

pub fn run_monitor(vault: Arc<Mutex<SecureVault>>) {
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            println!("[ERR] Failed to initialize Clipboard API: {}", e);
            return;
        }
    };
    
    let mut last_content = String::new();
    println!("[DEBUG] Clipboard monitor thread successfully initialized.");

   loop {
        sleep(Duration::from_millis(500));

        match clipboard.get_text() {
            Ok(current_content) => {
                if current_content.is_empty() {
                    continue;
                }
                
                if current_content == last_content {
                    continue;
                }

                last_content = current_content.clone();

                let entropy = calculate_entropy(&current_content);
                let len = current_content.len();
                let has_space = current_content.contains(' ');

                println!(
                    "[DEBUG] Detected change. Len: {}, Entropy: {:.2}, Has Space: {}", 
                    len, entropy, has_space
                );

                let cfg = crate::config::AppConfig::load();
                if entropy > cfg.min_entropy && len >= cfg.min_length && !has_space {
                    println!("[!] Criteria matched. Locking vault...");

                    match vault.try_lock() {
                        Ok(mut v) => {
                            if v.protect(&current_content).is_ok() {
                                if clipboard.set_text("").is_ok() {
                                    println!("[+] Clipboard wiped. Data secured.");
                                    last_content = String::new();
                                } else {
                                    println!("[ERR] Failed to wipe clipboard");
                                }
                            } else {
                                println!("[ERR] Encryption failed");
                            }
                        }
                        Err(_) => {
                            println!("[ERR] Vault is locked by another thread");
                        }
                    }
                } else {
                    println!("[DEBUG] Ignored: does not match high-entropy criteria.");
                }
            }
            Err(e) => {
                println!("[ERR] Failed to read clipboard text: {}", e);
            }
        }
    }
}