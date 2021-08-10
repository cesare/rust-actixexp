use actix_service::ServiceFactory;
use actix_web::{Error, HttpResponse, Scope};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::{Data, get, scope};
use serde_json::json;

use crate::app::config::ApplicationConfig;
use crate::app::Result;

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl ServiceFactory<ServiceRequest, InitError = (), Error = Error, Response = ServiceResponse, Config = ()>> {
    scope("/auth")
        .app_data(Data::new(config.clone()))
        .route("", get().to(start))
}

async fn start(_config: Data<ApplicationConfig>) -> Result<HttpResponse> {
    let response_json = json!({"result": "ok"});
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
