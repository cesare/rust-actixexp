use deadpool_postgres::Pool;

use crate::app::config::ApplicationConfig;

use anyhow::Result;

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
        let pool = config.database.create_pool()?;
        Ok(Self::new(pool))
    }
}
