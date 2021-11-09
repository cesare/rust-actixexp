use anyhow::Result;

use super::{config::ApplicationConfig, db::connection::DatabaseConnection};

#[derive(Clone)]
pub struct Context {
    pub config: ApplicationConfig,
    pub db: DatabaseConnection,
}

impl Context {
    pub fn initialize(config: &ApplicationConfig) -> Result<Self> {
        let db = DatabaseConnection::initialize(&config.database)?;
        let context = Self {
            config: config.clone(),
            db: db,
        };
        Ok(context)
    }
}
