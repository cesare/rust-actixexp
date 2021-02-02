use actix_web::{delete, get, post, web, HttpResponse};
use anyhow::Result;
use deadpool_postgres::{Pool};

use crate::app::db::{CreateServantRequest, ServantRepository};
use crate::app::errors::ActixexpError;

#[post("/servants")]
pub async fn create(db_pool: web::Data<Pool>, form: web::Form<CreateServantRequest>) -> Result<HttpResponse, ActixexpError> {
    let repository = create_repository(db_pool).await?;
    let result = repository.create(form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

#[get["/servants"]]
pub async fn list(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixexpError> {
    let repository = create_repository(db_pool).await?;
    let results = repository.list().await?;
    let response = HttpResponse::Ok().json(results);
    Ok(response)
}

#[get("/servants/{id}")]
pub async fn show(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let repository = create_repository(db_pool).await?;
    let result = repository.show(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[delete("/servants/{id}")]
pub async fn destroy(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let repository = create_repository(db_pool).await?;
    let result = repository.delete(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn create_repository(pool: web::Data<Pool>) -> Result<ServantRepository, ActixexpError> {
    let client = pool.get().await?;
    Ok(ServantRepository::new(client))
}
