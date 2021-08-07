use anyhow::Result;
use structopt::StructOpt;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use std::path::PathBuf;

mod database;
mod frontend;
mod server;

use self::database::DatabaseConfig;
use self::frontend::FrontendConfig;
use self::server::ServerConfig;

#[derive(StructOpt)]
#[structopt(name = "actixexp")]
pub struct AppArgs {
    #[structopt(short, long, parse(from_os_str))]
    config_file: PathBuf,
}

impl AppArgs {
    pub fn new() -> Self {
        Self::from_args()
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
    pub database: DatabaseConfig,
    pub frontend: FrontendConfig,
}
