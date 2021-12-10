use anyhow::Result;

use super::{config::ApplicationConfig, db::connection::RepositoryAccess};

#[derive(Clone)]
pub struct Context {
    pub config: ApplicationConfig,
    pub db: RepositoryAccess,
}

impl Context {
    pub fn initialize(config: &ApplicationConfig) -> Result<Self> {
        let db = RepositoryAccess::initialize(&config.database)?;
        let context = Self {
            config: config.clone(),
            db: db,
        };
        Ok(context)
    }
}
