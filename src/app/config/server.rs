use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    bind: String,
    port: u32,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

