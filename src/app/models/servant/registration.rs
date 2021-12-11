use crate::app::context::Context;
use crate::app::db::servant_repository::{RegistrationDataset, ServantRepository};
use crate::app::models::DomainError;
use super::Servant;

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

    pub async fn execute(self) -> Result<Servant, DomainError> {
        let connection = self.context.db.establish_connection().await?;
        let repository = ServantRepository::new(&connection);
        let dataset = RegistrationDataset {
            name: self.name,
            class_name: self.class_name,
        };
        let servant = repository.create(dataset).await?;
        Ok(servant)
    }
}
