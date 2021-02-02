use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActixexpError {
    #[error("Not Found")]
    NotFound,
    #[error("connection pool faild")]
    PoolFailed(#[from] deadpool_postgres::PoolError),
    #[error("query failed")]
    QueryFailed(#[from] tokio_postgres::Error),
}

impl ResponseError for ActixexpError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ActixexpError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
