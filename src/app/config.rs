use deadpool_postgres::Config as DeadpoolConfig;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub struct ActixexpConfig {
    pool_config: DeadpoolConfig,
}

impl ActixexpConfig {
    pub fn new() -> Self {
        ActixexpConfig {
            pool_config: create_pool_config(),
        }
    }

    pub fn create_pool(self) -> Pool {
        self.pool_config.create_pool(NoTls).unwrap()
    }
}

pub fn create_pool_config() -> DeadpoolConfig {
    let mut config = DeadpoolConfig::new();
    config.host     = Some("localhost".to_string());
    config.dbname   = Some("actixexp".to_string());
    config.user     = std::env::var("POSTGRES_USER").ok();
    config.password = std::env::var("POSTGRES_PASSWORD").ok();

    let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    config.manager = Some(manager_config);

    config
}
