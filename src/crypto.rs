use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct EncryptionKey {
    pub key: [u8; 32],
}

pub struct SecureVault {
    enc_key: EncryptionKey,
    encrypted_data: Option<Vec<u8>>,
    nonce: Option<[u8; 24]>,
}

impl SecureVault {
    pub fn new() -> Self {
        let mut raw_key = [0u8; 32];
        OsRng.fill_bytes(&mut raw_key);
        Self {
            enc_key: EncryptionKey { key: raw_key },
            encrypted_data: None,
            nonce: None,
        }
    }

    pub fn has_secret(&self) -> bool {
        self.encrypted_data.is_some()
    }

    pub fn protect(&mut self, secret: &str) -> Result<(), chacha20poly1305::Error> {
        let cipher = XChaCha20Poly1305::new((&self.enc_key.key).into());
        let mut nonce_bytes = [0u8; 24];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, secret.as_bytes())?;
        self.encrypted_data = Some(ciphertext);
        self.nonce = Some(nonce_bytes);
        Ok(())
    }

    pub fn reveal(&mut self) -> Result<String, ()> {
        let data = self.encrypted_data.take().ok_or(())?;
        let nonce_bytes = self.nonce.take().ok_or(())?;
        
        let cipher = XChaCha20Poly1305::new((&self.enc_key.key).into());
        let nonce = XNonce::from_slice(&nonce_bytes);

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

pub fn calculate_entropy(s: &str) -> f64 {
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