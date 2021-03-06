use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
pub struct Servant {
  id: i32,
  name: String,
  class: String,
}
