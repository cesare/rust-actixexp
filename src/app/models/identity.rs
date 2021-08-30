use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "identities")]
pub struct Identity {
    id: String,
    provider_identifier: String,
    alive: bool,
}
