use serde::{Deserialize, Serialize, Serializer};
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

impl From<&ServantClass> for String {
    fn from(clazz: &ServantClass) -> String {
        let string_representation = match clazz {
            ServantClass::Saber      => "saber",
            ServantClass::Archer     => "archer",
            ServantClass::Lancer     => "lancer",
            ServantClass::Rider      => "rider",
            ServantClass::Caster     => "caster",
            ServantClass::Assassin   => "assassin",
            ServantClass::Berserker  => "berserker",
            ServantClass::Ruler      => "ruler",
            ServantClass::Avenger    => "avenger",
            ServantClass::Mooncancer => "mooncancer",
            ServantClass::Alterego   => "alterego",
            ServantClass::Foreigner  => "foreigner",
            ServantClass::Pretender  => "pretender",
            ServantClass::Shielder   => "shielder",
        };
        string_representation.to_owned()
    }
}

impl TryFrom<&str> for ServantClass {
    type Error = ServantClassConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "saber"      => Ok(Self::Saber),
            "archer"     => Ok(Self::Archer),
            "lancer"     => Ok(Self::Lancer),
            "rider"      => Ok(Self::Rider),
            "caster"     => Ok(Self::Caster),
            "assassin"   => Ok(Self::Assassin),
            "berserker"  => Ok(Self::Berserker),
            "ruler"      => Ok(Self::Ruler),
            "avenger"    => Ok(Self::Avenger),
            "mooncancer" => Ok(Self::Mooncancer),
            "alterego"   => Ok(Self::Alterego),
            "foreigner"  => Ok(Self::Foreigner),
            "pretender"  => Ok(Self::Pretender),
            "shielder"   => Ok(Self::Shielder),
            _ => Err(ServantClassConversionError::UnknownClass)
        }
    }
}

impl Serialize for ServantClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name: String = self.into();
        serializer.serialize_str(&name)
    }
}

pub enum ServantClassConversionError {
    UnknownClass,
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
