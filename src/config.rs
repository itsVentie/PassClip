use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: String,
    pub min_entropy: f64,
    pub min_length: usize,
    pub zeroize_timeout_secs: u64,
    pub enable_notifications: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self{
            rp_id: "localhost".to_string(),
            rp_origin: "http://localhost".to_string(),
            rp_name: "PassClip Vault".to_string(),
            min_entropy: 4.5,
            min_length: 8,
            zeroize_timeout_secs: 30,
            enable_notifications: true,
        }
    }
}

impl AppConfig {
    fn get_config_path() -> PathBuf {
        let mut path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();
        path.push("passclip.toml");
        path
    }
    pub fn load() -> Self {
        let path = Self::get_config_path();
        if !path.exists() {
            let default_config = Self::default();
            if let Ok(toml_str) = toml::to_string_pretty(&default_config) {
                let _ = fs::write(&path, toml_str);
            }
            return default_config;
        }

        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_else(|_| Self::default())
    }
}