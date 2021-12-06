use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::app::context::Context;
use crate::app::db::servant_repository::{RegistrationDataset, ServantRepository};

use super::DomainError;

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

pub struct ServantRegistration<'a> {
    context: &'a Context,
    name: String,
    class_name: String,
}

impl<'a> ServantRegistration<'a> {
    pub fn new(context: &'a Context, name: &str, class_name: &str) -> Self {
      Self {
          context: context,
          name: name.to_owned(),
          class_name: class_name.to_owned(),
      }
    }

    pub async fn execute(&self) -> Result<Servant, DomainError> {
        let repository = ServantRepository::initialize(&self.context.db).await?;
        let dataset = RegistrationDataset {
            name: self.name.to_owned(),
            class_name: self.class_name.to_owned(),
        };
        let servant = repository.create(dataset).await?;
        Ok(servant)
    }
}

pub struct ServantListing<'a> {
    context: &'a Context,
}

impl<'a> ServantListing<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context: context,
        }
    }

    pub async fn execute(&self) -> Result<Vec<Servant>, DomainError> {
        let repository = ServantRepository::initialize(&self.context.db).await?;
        let servants = repository.list().await?;
        Ok(servants)
    }
}

pub struct ServantFetching<'a> {
    context: &'a Context,
    id: i32,
}

impl<'a> ServantFetching<'a> {
    pub fn new(context: &'a Context, id: i32) -> Self {
        Self {
            context: context,
            id: id,
        }
    }

    pub async fn execute(&self) -> Result<Servant, DomainError> {
        let repository = ServantRepository::initialize(&self.context.db).await?;
        let servants = repository.show(self.id).await?;
        Ok(servants)
    }
}

pub struct ServantDeletion<'a> {
    context: &'a Context,
    id: i32,
}

impl<'a> ServantDeletion<'a> {
    pub fn new(context: &'a Context, id: i32) -> Self {
        Self {
            context: context,
            id: id,
        }
    }

    pub async fn execute(&self) -> Result<Servant, DomainError> {
        let repository = ServantRepository::initialize(&self.context.db).await?;
        let servant = repository.delete(self.id).await?;
        Ok(servant)
    }
}
