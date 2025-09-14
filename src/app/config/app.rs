use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    session_key: String,
}

impl AppConfig {
    pub fn raw_session_key(&self) -> Result<Vec<u8>> {
        STANDARD.decode(&self.session_key)
            .with_context(|| "Failed to parse session_key as Base64 string")
    }
}
