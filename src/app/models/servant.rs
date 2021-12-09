use serde::{Deserialize, Serialize};

pub use crate::app::db::servant_repository::Servant;

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
