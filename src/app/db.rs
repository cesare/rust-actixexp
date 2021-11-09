use thiserror::Error;

pub mod connection;
pub mod identity_repository;
pub mod servant_repository;

pub use servant_repository::{CreateServantRequest, ServantRepository};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to initialize connection: {source}")]
    InitializationFailed {
        #[source]
        source: deadpool_postgres::CreatePoolError,
    },

    #[error("Failed to establish connection: {source}")]
    EstablishFailed {
        #[source]
        source: deadpool_postgres::PoolError,
    },

    #[error("Not found")]
    NotFound,

    #[error("Query failed: {source}")]
    QueryFailed {
        #[source]
        source: tokio_postgres::error::Error,
    },
}
