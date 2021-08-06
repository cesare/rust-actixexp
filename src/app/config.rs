use anyhow::Result;
use deadpool_postgres::Config as DeadpoolConfig;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use structopt::StructOpt;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_postgres::NoTls;

use std::env;
use std::path::PathBuf;

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
pub struct ServerConfig {
    bind: String,
    port: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u32,
    database: String,
    user: String,
    password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FrontendConfig {
    base_uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub frontend: FrontendConfig,
}

#[derive(Clone)]
pub struct ActixexpConfig {
    pub app_config: AppConfig,
    pool_config: DeadpoolConfig,
}

impl ActixexpConfig {
    pub fn new() -> Self {
        ActixexpConfig {
            app_config: AppConfig::from_args(),
            pool_config: create_pool_config(),
        }
    }

    pub fn create_pool(&self) -> Pool {
        self.pool_config.create_pool(NoTls).unwrap()
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.app_config.bind, self.app_config.port)
    }
}

#[derive(Clone, StructOpt)]
#[structopt(name = "actixexp")]
pub struct AppConfig {
    #[structopt(short = "b", long = "bind", default_value = "127.0.0.1")]
    bind: String,

    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: u32,

    #[structopt(short = "u", long = "frontend-base-uri", default_value = "http://localhost:3000")]
    pub frontend_base_uri: String,
}

pub fn create_pool_config() -> DeadpoolConfig {
    let mut config = DeadpoolConfig::new();
    config.host     = env::var("POSTGRES_HOST").ok().or(Some("localhost".to_string()));
    config.dbname   = Some("actixexp".to_string());
    config.user     = env::var("POSTGRES_USER").ok();
    config.password = env::var("POSTGRES_PASSWORD").ok();

    let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    config.manager = Some(manager_config);

    config
}
