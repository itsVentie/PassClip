use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternRule {
    pub name: String,
    pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RulesConfig {
    pub version: u32,
    pub rules: Vec<PatternRule>,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            version: 1,
            rules: vec![
                PatternRule {
                    name: "AWS Access Key".into(),
                    pattern: r"AKIA[0-9A-Z]{16}".into(),
                },
                PatternRule {
                    name: "GitHub Personal Access Token".into(),
                    pattern: r"ghp_[a-zA-Z0-9]{36}".into(),
                },
                PatternRule {
                    name: "GitLab Personal Access Token".into(),
                    pattern: r"glpat-[a-zA-Z0-9\-]{20}".into(),
                },
                PatternRule {
                    name: "PEM Private Key".into(),
                    pattern: r"-----BEGIN [A-Z ]+ PRIVATE KEY-----".into(),
                },
                PatternRule {
                    name: "JWT Header".into(),
                    pattern: r"eyJhbGciOiJSUzI1NiIsI".into(),
                },
            ],
        }
    }
}

impl RulesConfig {
    fn get_rules_path() -> PathBuf {
        let root_path = PathBuf::from("rules.toml");
        if root_path.exists() {
            return root_path;
        }

        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("."))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .join("rules.toml");

        if exe_path.exists() {
            return exe_path;
        }

        root_path
    }

    pub fn load() -> Self {
        let path = Self::get_rules_path();

        if !path.exists() {
            let default_rules = Self::default();
            if let Ok(toml_str) = toml::to_string_pretty(&default_rules) {
                if let Err(e) = fs::write(&path, toml_str) {
                    warn!("Failed to write default rules.toml: {}", e);
                } else {
                    info!("Created default rules.toml at {:?}", path);
                }
            }
            return default_rules;
        }

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
                warn!(
                    "Failed to parse rules.toml: {}. Falling back to defaults.",
                    e
                );
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    pub fn compile_patterns(&self) -> Vec<String> {
        self.rules.iter().map(|r| r.pattern.clone()).collect()
    }
}
