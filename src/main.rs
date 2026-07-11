use arboard::Clipboard;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use clap::{Parser, Subcommand};
use rand::{rngs::OsRng, RngCore};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
struct EncryptionKey {
    key: [u8; 32],
}

struct SecureVault {
    enc_key: EncryptionKey,
    encrypted_data: Option<Vec<u8>>,
    nonce: Option<[u8; 24]>,
}

impl SecureVault {
    fn new() -> Self {
        let mut raw_key = [0u8; 32];
        OsRng.fill_bytes(&mut raw_key);
        Self {
            enc_key: EncryptionKey { key: raw_key },
            encrypted_data: None,
            nonce: None,
        }
    }

    fn protect(&mut self, secret: &str) -> Result<(), chacha20poly1305::Error> {
        let cipher = XChaCha20Poly1305::new((&self.enc_key.key).into());
        let mut nonce_bytes = [0u8; 24];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, secret.as_bytes())?;
        self.encrypted_data = Some(ciphertext);
        self.nonce = Some(nonce_bytes);
        Ok(())
    }

    fn reveal(&self) -> Result<String, ()> {
        let data = self.encrypted_data.as_ref().ok_or(())?;
        let nonce_bytes = self.nonce.as_ref().ok_or(())?;
        
        let cipher = XChaCha20Poly1305::new((&self.enc_key.key).into());
        let nonce = XNonce::from_slice(nonce_bytes);

        if let Ok(mut decrypted_bytes) = cipher.decrypt(nonce, data.as_slice()) {
            if let Ok(plaintext) = String::from_utf8(decrypted_bytes.clone()) {
                decrypted_bytes.zeroize();
                return Ok(plaintext);
            }
            decrypted_bytes.zeroize();
        }
        Err(())
    }
}

fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut frequencies = HashMap::new();
    for c in s.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }
    let len = s.len() as f64;
    frequencies.values().map(|&count| {
        let p = count as f64 / len;
        -p * p.log2()
    }).sum()
}

#[derive(Parser)]
#[command(name = "passclip")]
#[command(about = "Secures clipboard data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Monitor,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Monitor => {
            let mut clipboard = Clipboard::new().unwrap();
            let mut vault = SecureVault::new();
            let mut last_content = String::new();

            println!("PassClip monitor active. Press Ctrl+C to stop.");

            loop {
                thread::sleep(Duration::from_millis(500));

                if let Ok(current_content) = clipboard.get_text() {
                    if current_content.is_empty() || current_content == last_content {
                        continue;
                    }

                    let entropy = calculate_entropy(&current_content);

                    if entropy > 4.5 && current_content.len() > 8 && !current_content.contains(' ') {
                        println!("[!] High entropy: {:.2}. Moving secret to RAM vault...", entropy);

                        if vault.protect(&current_content).is_ok() {
                            if clipboard.set_text("").is_ok() {
                                println!("[+] Clipboard wiped. Data is encrypted in memory.");
                                
                                if let Ok(check) = vault.reveal() {
                                    println!("[DEBUG] Vault integrity verified. (Len: {})", check.len());
                                }
                            }
                        }
                        last_content = String::new();
                    } else {
                        last_content = current_content;
                    }
                }
            }
        }
    }
}
