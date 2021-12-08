use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
pub struct Servant {
    id: i32,
    name: String,
    class_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServantClass {
    Saber,
    Archer,
    Lancer,
    Rider,
    Caster,
    Assassin,
    Berserker,
    Ruler,
    Avenger,
    Mooncancer,
    Alterego,
    Foreigner,
    Pretender,
    Shielder,
}

mod registration;
pub use registration::ServantRegistration;

mod listing;
pub use listing::ServantListing;

mod fetching;
pub use fetching::ServantFetching;

mod deletion;
pub use deletion::ServantDeletion;
