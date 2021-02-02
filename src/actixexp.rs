use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use anyhow::Result;
use deadpool_postgres::{Config, Client, ManagerConfig, Pool, RecyclingMethod};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::NoTls;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
struct Servant {
    id: i32,
    name: String,
    class: String,
}

#[derive(Error, Debug)]
enum ActixexpError {
    #[error("Not Found")]
    NotFound,
    #[error("connection pool faild")]
    PoolFailed(#[from] deadpool_postgres::PoolError),
    #[error("query failed")]
    QueryFailed(#[from] tokio_postgres::Error),
}

impl ResponseError for ActixexpError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ActixexpError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

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

#[derive(Deserialize)]
struct CreateServantRequest {
    name: String,
    class: String,
}

async fn create_servant(client: &Client, request: CreateServantRequest) -> Result<Servant, ActixexpError> {
    let rows = client.query("insert into servants (name, class) values ($1, $2) returning id, name, class", &[&request.name, &request.class]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}

async fn list_servants(client: &Client) -> Result<Vec<Servant>, ActixexpError> {
    let rows = client.query("select id, name, class from servants", &[]).await?;
    let results = rows.iter()
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect();
    Ok(results)
}

async fn show_servant(client: &Client, id: i32) -> Result<Servant, ActixexpError> {
    let rows = client.query("select id, name, class from servants where id = $1", &[&id]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}

async fn delete_servant(client: &Client, id: i32) -> Result<Servant, ActixexpError> {
    let rows = client.query("delete from servants where id = $1 returning id, name, class", &[&id]).await?;
    rows.iter()
        .take(1)
        .map(|row| Servant::from_row_ref(row).unwrap())
        .collect::<Vec<Servant>>()
        .pop()
        .ok_or(ActixexpError::NotFound)
}

#[post("/servants")]
async fn register_servant(db_pool: web::Data<Pool>, form: web::Form<CreateServantRequest>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = create_servant(&client, form.into_inner()).await?;
    let response = HttpResponse::Created().json(result);
    Ok(response)
}

#[get["/servants"]]
async fn servants(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let results = list_servants(&client).await?;
    let response = HttpResponse::Ok().json(results);
    Ok(response)
}

#[get("/servants/{id}")]
async fn servant(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = show_servant(&client, id).await?;
    let response = HttpResponse::Ok().json(result);
    Ok(response)
}

#[delete("/servants/{id}")]
async fn destroy_servant(db_pool: web::Data<Pool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, ActixexpError> {
    let client = db_pool.get().await?;
    let result = delete_servant(&client, id).await?;
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
