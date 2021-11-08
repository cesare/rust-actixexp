use thiserror::Error;

pub mod connection;
pub mod identity_repository;
pub mod servant_repository;

pub use servant_repository::{CreateServantRequest, ServantRepository};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to initialize connection")]
    InitializationFailed,

    #[error("Failed to establish connection")]
    EstablishFailed,

    #[error("Not found")]
    NotFound,

    #[error("Query failed: {source}")]
    QueryFailed {
        #[source]
        source: tokio_postgres::error::Error,
    },
}
