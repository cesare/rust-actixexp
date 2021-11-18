use thiserror::Error;

pub mod auth;
pub mod identity;
pub mod servant;

pub use identity::Identity;
pub use servant::Servant;

use super::db::DatabaseError;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Requested record is not found")]
    RecordNotFound,

    #[error("Database error: {source}")]
    DatabaseError {
        #[source]
        source: DatabaseError,
    }
}

impl From<DatabaseError> for DomainError {
    fn from(e: DatabaseError) -> Self {
        match e {
            DatabaseError::NotFound => Self::RecordNotFound,
            _ => Self::DatabaseError { source: e },
        }
    }
}
