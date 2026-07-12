pub struct AppConfig {
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: String,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            rp_id: "localhost".to_string(),
            rp_origin: "http://localhost".to_string(),
            rp_name: "PassClip Vault".to_string(),
        }
    }
}