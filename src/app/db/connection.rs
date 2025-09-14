use std::ops::Deref;

use deadpool_postgres::{Client, ManagerConfig, Pool, RecyclingMethod, Runtime};
use deadpool_postgres::Config as DeadpoolConfig;
use tokio_postgres::NoTls;

use crate::app::config::DatabaseConfig;

use super::DatabaseError;

type Result<T, E = DatabaseError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct RepositoryAccess {
    pool: Pool,
}

impl RepositoryAccess {
    fn new(pool: Pool) -> Self {
        Self {
            pool: pool,
        }
    }

    pub fn initialize(config: &DatabaseConfig) -> Result<Self> {
        let pool = Self::create_pool(&config)?;
        Ok(Self::new(pool))
    }

    fn create_pool(config: &DatabaseConfig) -> Result<Pool> {
        let mut pool_config = DeadpoolConfig::new();
        pool_config.host     = Some(config.host.to_owned());
        pool_config.port     = Some(config.port.to_owned());
        pool_config.dbname   = Some(config.database.to_owned());
        pool_config.user     = Some(config.user.to_owned());
        pool_config.password = Some(config.password.to_owned());

        let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
        pool_config.manager = Some(manager_config);

        pool_config.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| DatabaseError::InitializationFailed {source: e})
    }

    pub async fn establish_connection(&self) -> Result<DatabaseConnection> {
        let client = self.pool.get().await
            .map_err(|e| DatabaseError::EstablishFailed {source: e})?;
        let connection = DatabaseConnection::new(client);
        Ok(connection)
    }
}

pub struct DatabaseConnection {
    client: Client,
}

impl DatabaseConnection {
    fn new(client: Client) -> Self {
        Self {
            client: client,
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
