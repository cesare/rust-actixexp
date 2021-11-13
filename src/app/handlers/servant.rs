use std::sync::Arc;

use actix_cors::Cors;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::web::{delete, get, post};
use actix_web::{HttpResponse, Scope, web};
use serde_json::json;

use crate::app::config::ApplicationConfig;
use crate::app::context::Context;
use crate::app::db::{CreateServantRequest, ServantRepository};
use crate::app::middlewares::IdentityValidator;
use crate::app::models::servant::ServantRegistration;

type Ctx = web::Data<Arc<Context>>;
type Result<T, E = actix_web::Error> = std::result::Result<T, E>;

fn create_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_uri)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl actix_service::ServiceFactory<ServiceRequest, InitError = (), Error = actix_web::Error, Response = ServiceResponse, Config = ()>> {
    let cors = create_cors(config);
    let identity_validator = IdentityValidator::new(config);
    web::scope("/servants")
        .wrap(cors)
        .wrap(identity_validator)
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy))
}

async fn create(context: Ctx, form: web::Json<CreateServantRequest>) -> Result<HttpResponse> {
    let registration = ServantRegistration::new(&context, &form.name, &form.class_name);
    let servant = registration.execute().await?;
    let response = HttpResponse::Created().json(servant);
    Ok(response)
}

async fn list(context: Ctx) -> Result<HttpResponse> {
    let repository = create_repository(&context).await?;
    let results = repository.list().await?;
    let response_json = json!({
        "servants": results,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(context: Ctx, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let repository = create_repository(&context).await?;
    let result = repository.show(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn destroy(context: Ctx, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let repository = create_repository(&context).await?;
    let result = repository.delete(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn create_repository(context: &Ctx) -> Result<ServantRepository> {
    let repository = ServantRepository::initialize(&context.db).await?;
    Ok(repository)
}
