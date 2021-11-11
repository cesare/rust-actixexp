use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

use super::db::DatabaseError;

pub mod auth;
pub mod root;
pub mod servant;

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Self::NotFound => generic_not_found_response(),
            _ => generic_internal_server_error_response(),
        }
    }
}

fn generic_not_found_response() -> HttpResponse {
    let body = json!({
        "error": "not found",
    });
    HttpResponse::NotFound().json(body)
}

fn generic_internal_server_error_response() -> HttpResponse {
    let body = json!({
        "error": "internal server error",
    });
    HttpResponse::InternalServerError().json(body)
}
