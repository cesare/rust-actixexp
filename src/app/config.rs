use anyhow::Result;
use clap::Parser;
use serde_derive::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use std::path::PathBuf;

mod app;
mod auth;
mod database;
mod frontend;
mod server;

pub use self::app::AppConfig;
pub use self::auth::AuthConfig;
pub use self::database::DatabaseConfig;
pub use self::frontend::FrontendConfig;
pub use self::server::ServerConfig;

#[derive(Parser)]
pub struct AppArgs {
    #[arg(short, long)]
    config_file: PathBuf,
}

impl AppArgs {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn load_config(&self) -> Result<ApplicationConfig> {
        let mut file = File::open(&self.config_file).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config: ApplicationConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub app: AppConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub frontend: FrontendConfig,
}
