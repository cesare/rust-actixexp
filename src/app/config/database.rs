use deadpool_postgres::Config as DeadpoolConfig;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u32,
    database: String,
    user: String,
    password: String,
}

impl From<&DatabaseConfig> for DeadpoolConfig {
    fn from(conf: &DatabaseConfig) -> DeadpoolConfig {
        let mut config = DeadpoolConfig::new();
        config.host     = Some(conf.host.to_owned());
        config.dbname   = Some(conf.database.to_owned());
        config.user     = Some(conf.user.to_owned());
        config.password = Some(conf.password.to_owned());

        let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
        config.manager = Some(manager_config);

        config
    }
}

impl DatabaseConfig {
    pub fn create_pool(&self) -> Pool {
        let pool_config: DeadpoolConfig = self.into();
        pool_config.create_pool(NoTls).unwrap()
    }
}
