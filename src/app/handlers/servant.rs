use actix_cors::Cors;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::web::{delete, get, post};
use actix_web::{HttpResponse, Scope, web};
use deadpool_postgres::{Pool};
use serde_json::json;

use crate::app::Result;
use crate::app::config::ApplicationConfig;
use crate::app::middlewares::IdentityValidator;
use crate::app::db::{CreateServantRequest, ServantRepository};

type DbPool = web::Data<Pool>;

fn create_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_uri)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl actix_service::ServiceFactory<ServiceRequest, InitError = (), Error = actix_web::Error, Response = ServiceResponse, Config = ()>> {
    let cors = create_cors(config);
    let identity_validator = IdentityValidator{};
    web::scope("/servants")
        .wrap(cors)
        .wrap(identity_validator)
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy))
}

async fn create(db_pool: DbPool, form: web::Json<CreateServantRequest>) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let result = repository.create(form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

async fn list(db_pool: DbPool) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let results = repository.list().await?;
    let response_json = json!({
        "servants": results,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(db_pool: DbPool, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let repository = create_repository(db_pool).await?;
    let result = repository.show(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn destroy(db_pool: DbPool, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let repository = create_repository(db_pool).await?;
    let result = repository.delete(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn create_repository(pool: DbPool) -> Result<ServantRepository> {
    let client = pool.get().await?;
    Ok(ServantRepository::new(client))
}
