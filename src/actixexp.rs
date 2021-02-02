use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

mod app;
use self::app::db::{self};
use self::app::errors::ActixexpError;

fn create_pool_config() -> Config {
    let mut config = Config::new();
    config.host     = Some("localhost".to_string());
    config.dbname   = Some("actixexp".to_string());
    config.user     = std::env::var("POSTGRES_USER").ok();
    config.password = std::env::var("POSTGRES_PASSWORD").ok();

    let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
    config.manager = Some(manager_config);

    config
}

#[post("/servants")]
async fn register_servant(db_pool: web::Data<Pool>, form: web::Form<db::CreateServantRequest>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = db::create_servant(&client, form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

#[get["/servants"]]
async fn servants(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let results = db::list_servants(&client).await?;
    let response = HttpResponse::Ok().json(results);
    Ok(response)
}

#[get("/servants/{id}")]
async fn servant(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = db::show_servant(&client, id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[delete("/servants/{id}")]
async fn destroy_servant(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = db::delete_servant(&client, id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool_config = create_pool_config();
    let pool = pool_config.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(register_servant)
            .service(servants)
            .service(servant)
            .service(destroy_servant)
    });
    server.bind("127.0.0.1:8000")?.run().await
}
