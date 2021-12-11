use crate::app::context::Context;
use crate::app::db::servant_repository::ServantRepository;
use crate::app::models::DomainError;
use super::Servant;
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
        let connection = self.context.db.establish_connection().await?;
        let repository = ServantRepository::new(&connection);
        let servants = repository.show(self.id).await?;
        Ok(servants)
    }
}
