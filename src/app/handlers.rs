use actix_web::{HttpResponse, ResponseError};

use super::db::DatabaseError;

pub mod auth;
pub mod root;
pub mod servant;

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
