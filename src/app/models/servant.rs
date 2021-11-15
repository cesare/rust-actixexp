use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::app::{context::Context, db::{CreateServantRequest, DatabaseError, ServantRepository}};

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
pub struct Servant {
  id: i32,
  name: String,
  class_name: String,
}

pub struct ServantRegistration {
  context: Arc<Context>,
  name: String,
  class_name: String,
}

impl ServantRegistration {
  pub fn new(context: &Arc<Context>, name: &str, class_name: &str) -> Self {
    Self {
      context: context.clone(),
      name: name.to_owned(),
      class_name: class_name.to_owned(),
    }
  }

  pub async fn execute(&self) -> Result<Servant, DatabaseError> {
    let repository = ServantRepository::initialize(&self.context.db).await?;
    let request = CreateServantRequest {
      name: self.name.to_owned(),
      class_name: self.class_name.to_owned(),
    };
    let servant = repository.create(request).await?;
    Ok(servant)
  }
}

pub struct ServantListing {
  context: Arc<Context>,
}

impl ServantListing {
  pub fn new(context: &Arc<Context>) -> Self {
    Self {
      context: context.clone(),
    }
  }

  pub async fn execute(&self) -> Result<Vec<Servant>, DatabaseError> {
    let repository = ServantRepository::initialize(&self.context.db).await?;
    let servants = repository.list().await?;
    Ok(servants)
  }
}

pub struct ServantFetching {
  context: Arc<Context>,
  id: i32,
}

impl ServantFetching {
  pub fn new(context: &Arc<Context>, id: i32) -> Self {
    Self {
      context: context.clone(),
      id: id,
    }
  }

  pub async fn execute(&self) -> Result<Servant, DatabaseError> {
    let repository = ServantRepository::initialize(&self.context.db).await?;
    let servants = repository.show(self.id).await?;
    Ok(servants)
  }
}

pub struct ServantDeletion {
  context: Arc<Context>,
  id: i32,
}

impl ServantDeletion {
  pub fn new(context: &Arc<Context>, id: i32) -> Self {
    Self {
      context: context.clone(),
      id: id,
    }
  }

  pub async fn execute(&self) -> Result<Servant, DatabaseError> {
    let repository = ServantRepository::initialize(&self.context.db).await?;
    let servant = repository.delete(self.id).await?;
    Ok(servant)
  }
}
