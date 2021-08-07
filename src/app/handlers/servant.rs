use actix_http::Method;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use actix_web::web::{delete, get, post};
use actix_web::{HttpResponse, Route, Scope, web};
use deadpool_postgres::{Pool};
use serde_json::json;

use crate::app::Result;
use crate::app::config::ApplicationConfig;
use crate::app::db::{CreateServantRequest, ServantRepository};

type DbPool = web::Data<Pool>;

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl actix_service::ServiceFactory<ServiceRequest, InitError = (), Error = actix_web::Error, Response = ServiceResponse, Config = ()>> {
    let options_route = Route::new().method(Method::OPTIONS).to(options);
    let cors_headers = DefaultHeaders::new()
        .header("Access-Control-Allow-Origin", &config.frontend.base_uri)
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    web::scope("/servants")
        .wrap(cors_headers)
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy))
        .route("", options_route)
}

async fn options(_db_pool: DbPool) -> Result<HttpResponse> {
    let response = HttpResponse::NoContent().finish();
    Ok(response)
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
