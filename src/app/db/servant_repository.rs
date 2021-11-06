use deadpool_postgres::Client;
use serde::Deserialize;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::ToStatement;
use tokio_postgres::types::ToSql;

use crate::app::models::Servant;

use super::DatabaseError;
use super::connection::DatabaseConnection;

type Result<T, E = DatabaseError> = std::result::Result<T, E>;

#[derive(Deserialize)]
pub struct CreateServantRequest {
    name: String,
    class_name: String,
}

pub struct ServantRepository {
    client: Client,
}

impl ServantRepository {
    pub async fn initialize(connection: &DatabaseConnection) -> Result<Self> {
        let client = connection.establish().await?;
        let repository = Self {
            client: client,
        };
        Ok(repository)
    }

    pub async fn create(&self, request: CreateServantRequest) -> Result<Servant> {
        self.query("insert into servants (name, class_name) values ($1, $2) returning id, name, class_name", &[&request.name, &request.class_name]).await?
            .pop()
            .ok_or(DatabaseError::NotFound)
    }

    pub async fn list(&self) -> Result<Vec<Servant>> {
        self.query("select id, name, class_name from servants", &[]).await
    }

    pub async fn show(&self, id: i32) -> Result<Servant> {
        self.query("select id, name, class_name from servants where id = $1", &[&id]).await?
            .pop()
            .ok_or(DatabaseError::NotFound)
    }

    pub async fn delete(&self, id: i32) -> Result<Servant> {
        self.query("delete from servants where id = $1 returning id, name, class_name", &[&id]).await?
            .pop()
            .ok_or(DatabaseError::NotFound)
    }

    async fn query<T: ?Sized + ToStatement>(&'_ self, statement: &'_ T, params: &'_ [&'_ (dyn ToSql + Sync)]) -> Result<Vec<Servant>> {
        let rows = self.client.query(statement, params).await
            .or(Err(DatabaseError::QueryFailed))?;

        let servants = rows.iter()
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect::<Vec<Servant>>();
        Ok(servants)
    }
}
