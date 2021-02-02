use deadpool_postgres::Client;
use serde::Deserialize;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::errors::ActixexpError;
use super::models::Servant;

#[derive(Deserialize)]
pub struct CreateServantRequest {
    name: String,
    class: String,
}

pub async fn create_servant(client: &Client, request: CreateServantRequest) -> Result<Servant, ActixexpError> {
    let rows = client.query("insert into servants (name, class) values ($1, $2) returning id, name, class", &[&request.name, &request.class]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}

pub async fn list_servants(client: &Client) -> Result<Vec<Servant>, ActixexpError> {
    let rows = client.query("select id, name, class from servants", &[]).await?;
    let results = rows.iter()
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect();
    Ok(results)
}

pub async fn show_servant(client: &Client, id: i32) -> Result<Servant, ActixexpError> {
    let rows = client.query("select id, name, class from servants where id = $1", &[&id]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}

pub async fn delete_servant(client: &Client, id: i32) -> Result<Servant, ActixexpError> {
    let rows = client.query("delete from servants where id = $1 returning id, name, class", &[&id]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}
