use actix_service::ServiceFactory;
use actix_session::Session;
use actix_web::{Error, HttpResponse, Scope};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::{Data, get, scope};
use serde_json::json;

use crate::app::models::auth::AuthorizationRequest;
use crate::app::config::ApplicationConfig;
use crate::app::Result;

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl ServiceFactory<ServiceRequest, InitError = (), Error = Error, Response = ServiceResponse, Config = ()>> {
    scope("/auth")
        .app_data(Data::new(config.clone()))
        .route("", get().to(start))
}

async fn start(config: Data<ApplicationConfig>, session: Session) -> Result<HttpResponse> {
    let auth_request = AuthorizationRequest::new();
    session.insert("auth-state", &auth_request.state).unwrap(); // TODO: handle errors

    let response_json = json!({
        "client_id": &config.auth.client_id,
        "scope": "read:user",
        "state": &auth_request.state,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
