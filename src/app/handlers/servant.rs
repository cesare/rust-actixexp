use actix_web::{delete, get, post, web, HttpResponse};
use anyhow::Result;
use deadpool_postgres::{Pool};

use crate::app::db::{CreateServantRequest, ServantRepository};
use crate::app::errors::ActixexpError;

#[post("/servants")]
pub async fn create(db_pool: web::Data<Pool>, form: web::Form<CreateServantRequest>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = ServantRepository::new(client).create(form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

#[get["/servants"]]
pub async fn list(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let results = ServantRepository::new(client).list().await?;
    let response = HttpResponse::Ok().json(results);
    Ok(response)
}

#[get("/servants/{id}")]
pub async fn show(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = ServantRepository::new(client).show(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[delete("/servants/{id}")]
pub async fn destroy(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = ServantRepository::new(client).delete(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}
