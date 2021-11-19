use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;

use crate::app::models::Servant;

use super::DatabaseError;
use super::connection::DatabaseConnection;

type Result<T, E = DatabaseError> = std::result::Result<T, E>;

pub struct RegistrationDataset {
    pub name: String,
    pub class_name: String,
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

    pub async fn create(&self, dataset: RegistrationDataset) -> Result<Servant> {
        let statement = "insert into servants (name, class_name) values ($1, $2) returning id, name, class_name";
        let row = self.client.query_one(statement, &[&dataset.name, &dataset.class_name]).await?;
        row.try_into()
    }

    pub async fn list(&self) -> Result<Vec<Servant>> {
        let statement = "select id, name, class_name from servants";
        let rows = self.client.query(statement, &[]).await?;

        let servants = rows.iter()
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect::<Vec<Servant>>();
        Ok(servants)
    }

    pub async fn show(&self, id: i32) -> Result<Servant> {
        let statement = "select id, name, class_name from servants where id = $1";
        let row = self.client.query_opt(statement, &[&id]).await?
            .ok_or(DatabaseError::NotFound)?;
        row.try_into()
    }

    pub async fn delete(&self, id: i32) -> Result<Servant> {
        let statement = "delete from servants where id = $1 returning id, name, class_name";
        let row = self.client.query_opt(statement, &[&id]).await?
            .ok_or(DatabaseError::NotFound)?;
        row.try_into()
    }
}

impl TryFrom<Row> for Servant {
    type Error = DatabaseError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self::from_row(value)?)
    }
}
