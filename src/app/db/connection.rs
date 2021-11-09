use anyhow::Context;
use deadpool_postgres::{Client, ManagerConfig, Pool, RecyclingMethod};
use deadpool_postgres::Config as DeadpoolConfig;
use tokio_postgres::NoTls;

use crate::app::config::{ApplicationConfig, DatabaseConfig};

use super::DatabaseError;

type Result<T, E = DatabaseError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Pool,
}

impl DatabaseConnection {
    fn new(pool: Pool) -> Self {
        Self {
            pool: pool,
        }
    }

    pub fn initialize(config: &ApplicationConfig) -> Result<Self> {
        let pool = Self::create_pool(&config.database)
            .or(Err(DatabaseError::InitializationFailed))?;
        Ok(Self::new(pool))
    }

    pub fn create_pool(config: &DatabaseConfig) -> anyhow::Result<Pool> {
        let mut pool_config = DeadpoolConfig::new();
        pool_config.host     = Some(config.host.to_owned());
        pool_config.dbname   = Some(config.database.to_owned());
        pool_config.user     = Some(config.user.to_owned());
        pool_config.password = Some(config.password.to_owned());

        let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
        pool_config.manager = Some(manager_config);

        pool_config.create_pool(NoTls)
            .with_context(|| "Failed to create database pool")
    }

    pub async fn establish(&self) -> Result<Client> {
        let client = self.pool.get().await
            .map_err(|e| DatabaseError::EstablishFailed {source: e})?;
        Ok(client)
    }
}
