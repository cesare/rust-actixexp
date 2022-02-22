use serde_derive::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "identities")]
pub struct Identity {
    pub id: String,
    pub provider_identifier: String,
    pub alive: bool,
}
