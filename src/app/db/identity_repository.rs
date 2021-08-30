use deadpool_postgres::Client;
use thiserror::Error;

use crate::app::models::Identity;

#[derive(Debug, Error)]
pub enum IdentityRepositoryError {
}

type Result<T> = std::result::Result<T, IdentityRepositoryError>;

pub struct IdentityRepository {
    client: Client,
}

impl IdentityRepository {
    pub fn new(client: Client) -> Self {
        Self { client: client }
    }

    pub async fn find_by_provider_identifier(&self, identifier: &str) -> Result<Option<Identity>> {
        todo!()
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Identity> {
        todo!()
    }

    pub async fn create(&self, identifier: &str) -> Result<Identity> {
        todo!()
    }
}
