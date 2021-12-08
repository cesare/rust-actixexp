use crate::app::context::Context;
use crate::app::db::servant_repository::ServantRepository;
use crate::app::models::DomainError;
use super::Servant;

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
