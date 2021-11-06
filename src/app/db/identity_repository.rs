use deadpool_postgres::Client;
use thiserror::Error;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::app::models::Identity;

use super::{DatabaseError, connection::DatabaseConnection};

#[derive(Debug, Error)]
pub enum IdentityRepositoryError {
    #[error("Failed to query database")]
    QueryFailed(#[from] tokio_postgres::Error),

    #[error("Unknown row returned")]
    ObjectMappingFailed(#[from] tokio_pg_mapper::Error),
}

type Result<T> = std::result::Result<T, IdentityRepositoryError>;

pub struct IdentityRepository {
    client: Client,
}

impl IdentityRepository {
    pub async fn initialize(db: &DatabaseConnection) -> std::result::Result<Self, DatabaseError> {
        let client = db.establish().await?;
        let repository = Self {
            client: client,
        };
        Ok(repository)
    }

    pub async fn find_by_provider_identifier(&self, identifier: &str) -> Result<Option<Identity>> {
        let statement =
            "select cast(id as varchar) as id, provider_identifier, alive
                from identities where provider_identifier = $1
                limit 1";
        let result = self.client.query_opt(statement, &[&identifier]).await?;
        match result {
            Some(row) => {
                let identity = Identity::from_row_ref(&row)?;
                Ok(Some(identity))
            },
            None => {
                Ok(None)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_id(&self, id: &str) -> Result<Identity> {
        let statement =
            "select cast(id as varchar) as id, provider_identifier, alive
                from identities where id = $1
                limit 1";
        let row = self.client.query_one(statement, &[&id]).await?;
        let identity = Identity::from_row_ref(&row)?;
        Ok(identity)
    }

    pub async fn create(&self, identifier: &str) -> Result<Identity> {
        let statement =
            "insert into identities (id, provider_identifier)
               values (gen_random_uuid(), $1)
               returning id, provider_identifier, alive, registered_at";
        let row = self.client.query_one(statement, &[&identifier]).await?;
        let identity = Identity::from_row_ref(&row)?;
        Ok(identity)
    }

    pub async fn find_or_create(&self, provider_identifier: &str) -> Result<Identity> {
        let existing_identity = self.find_by_provider_identifier(provider_identifier).await?;
        match existing_identity {
            Some(identity) => Ok(identity),
            None => self.create(provider_identifier).await,
        }
    }
}
