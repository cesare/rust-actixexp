use actix_http::Method;
use actix_web::web::{delete, get, post};
use actix_web::{HttpResponse, Route, Scope, options, web};
use deadpool_postgres::{Pool};
use serde_json::json;

use crate::app::Result;
use crate::app::db::{CreateServantRequest, ServantRepository};

type DbPool = web::Data<Pool>;

pub fn create_scope() -> Scope {
    let options_route = Route::new().method(Method::OPTIONS).to(options);
    web::scope("/servants")
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy))
        .route("", options_route)
}

async fn options(_db_pool: DbPool) -> Result<HttpResponse> {
    let response = HttpResponse::NoContent()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .append_header(("Access-Control-Allow-Credentials", "true"))
        .finish();
    Ok(response)
}

async fn create(db_pool: DbPool, form: web::Json<CreateServantRequest>) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let result = repository.create(form.into_inner()).await?;
    let response = HttpResponse::Created()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:3000"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .append_header(("Access-Control-Allow-Credentials", "true"))
        .json(result);
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
