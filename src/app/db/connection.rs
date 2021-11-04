use deadpool_postgres::{Client, Pool};

use crate::app::config::ApplicationConfig;

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
        let pool = config.database.create_pool()
            .or(Err(DatabaseError::InitializationFailed))?;
        Ok(Self::new(pool))
    }

    pub async fn establish(&self) -> Result<Client> {
        let client = self.pool.get().await
            .or(Err(DatabaseError::EstablishFailed))?;
        Ok(client)
    }
}
