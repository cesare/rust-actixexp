use actix_web::{delete, get, post, web, HttpResponse};
use deadpool_postgres::{Pool};

use crate::app::Result;
use crate::app::db::{CreateServantRequest, ServantRepository};

type DbPool = web::Data<Pool>;

#[post("/servants")]
pub async fn create(db_pool: DbPool, form: web::Form<CreateServantRequest>) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let result = repository.create(form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

#[get["/servants"]]
pub async fn list(db_pool: DbPool) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let results = repository.list().await?;
    let response = HttpResponse::Ok().json(results);
    Ok(response)
}

#[get("/servants/{id}")]
pub async fn show(db_pool: DbPool, web::Path(id): web::Path<i32>) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let result = repository.show(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[delete("/servants/{id}")]
pub async fn destroy(db_pool: DbPool, web::Path(id): web::Path<i32>) -> Result<HttpResponse> {
    let repository = create_repository(db_pool).await?;
    let result = repository.delete(id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

async fn create_repository(pool: DbPool) -> Result<ServantRepository> {
    let client = pool.get().await?;
    Ok(ServantRepository::new(client))
}
