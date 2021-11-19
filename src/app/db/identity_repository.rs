use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;

use crate::app::models::Identity;

use super::{DatabaseError, connection::DatabaseConnection};

type Result<T, E = DatabaseError> = std::result::Result<T, E>;

pub struct IdentityRepository {
    client: Client,
}

impl IdentityRepository {
    pub async fn initialize(db: &DatabaseConnection) -> Result<Self> {
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
                let identity = row.try_into()?;
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
        row.try_into()
    }

    pub async fn create(&self, identifier: &str) -> Result<Identity> {
        let statement =
            "insert into identities (id, provider_identifier)
               values (gen_random_uuid(), $1)
               returning id, provider_identifier, alive, registered_at";
        let row = self.client.query_one(statement, &[&identifier]).await?;
        row.try_into()
    }

    pub async fn find_or_create(&self, provider_identifier: &str) -> Result<Identity> {
        let existing_identity = self.find_by_provider_identifier(provider_identifier).await?;
        match existing_identity {
            Some(identity) => Ok(identity),
            None => self.create(provider_identifier).await,
        }
    }
}

impl TryFrom<Row> for Identity {
    type Error = DatabaseError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self::from_row(value)?)
    }
}
