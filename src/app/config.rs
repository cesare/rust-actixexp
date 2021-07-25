use deadpool_postgres::Config as DeadpoolConfig;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use structopt::StructOpt;
use tokio_postgres::NoTls;

use std::env;

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
