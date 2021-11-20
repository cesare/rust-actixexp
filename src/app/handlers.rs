use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

use super::models::DomainError;

pub mod root;
mod auth;
mod servant;

pub use self::auth::auth_service_config;
pub use self::servant::servant_service_config;

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        self.create_response()
    }
}

impl DomainError {
    fn create_response(&self) -> HttpResponse {
        match self {
            Self::RecordNotFound => self.generic_not_found_response(),
            _ => self.generic_internal_server_error_response(),
        }
    }

    fn generic_not_found_response(&self) -> HttpResponse {
        let body = json!({
            "error": "not found",
        });
        HttpResponse::NotFound().json(body)
    }

    fn generic_internal_server_error_response(&self) -> HttpResponse {
        let body = json!({
            "error": "internal server error",
        });
        HttpResponse::InternalServerError().json(body)
    }
}
