use thiserror::Error;

pub mod connection;
pub mod identity_repository;
pub mod servant_repository;

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
        #[from]
        source: tokio_postgres::error::Error,
    },

    #[error("Failed to mapping row to object: {source}")]
    ObjectMappingFailed {
        #[from]
        source: tokio_pg_mapper::Error,
    }
}
