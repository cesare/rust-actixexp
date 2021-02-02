use deadpool_postgres::Client;
use serde::Deserialize;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::app::Result;
use crate::app::errors::ActixexpError;
use crate::app::models::Servant;

#[derive(Deserialize)]
pub struct CreateServantRequest {
    name: String,
    class: String,
}

pub struct ServantRepository {
    client: Client,
}

impl ServantRepository {
    pub fn new(client: Client) -> Self {
        ServantRepository {
            client: client,
        }
    }

    pub async fn create(self, request: CreateServantRequest) -> Result<Servant> {
        let rows = self.client.query("insert into servants (name, class) values ($1, $2) returning id, name, class", &[&request.name, &request.class]).await?;
        rows.iter()
            .take(1)
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect::<Vec<Servant>>()
            .pop()
            .ok_or(ActixexpError::NotFound)
    }

    pub async fn list(self) -> Result<Vec<Servant>> {
        let rows = self.client.query("select id, name, class from servants", &[]).await?;
        let results = rows.iter()
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect();
        Ok(results)
    }

    pub async fn show(self, id: i32) -> Result<Servant> {
        let rows = self.client.query("select id, name, class from servants where id = $1", &[&id]).await?;
        rows.iter()
            .take(1)
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect::<Vec<Servant>>()
            .pop()
            .ok_or(ActixexpError::NotFound)
    }

    pub async fn delete(self, id: i32) -> Result<Servant> {
        let rows = self.client.query("delete from servants where id = $1 returning id, name, class", &[&id]).await?;
        rows.iter()
            .take(1)
            .map(|row| Servant::from_row_ref(row).unwrap())
            .collect::<Vec<Servant>>()
            .pop()
            .ok_or(ActixexpError::NotFound)
    }
}
