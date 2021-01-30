use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Config, Client, ManagerConfig, Pool, RecyclingMethod};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, Error};

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
struct Servant {
    id: i32,
    name: String,
    class: String,
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

async fn list_servants(client: &Client) -> Result<Vec<Servant>, Error> {
    let rows = client.query("select id, name, class from servants", &[]).await?;
    let count = rows.len();
    let mut results = Vec::with_capacity(count);
    for row in rows {
        let result = Servant::from_row(row).unwrap();
        results.push(result);
    }
    Ok(results)
}

#[get["/servants"]]
async fn servants(db_pool: web::Data<Pool>) -> impl Responder {
    let client = db_pool.get().await.unwrap();
    match list_servants(&client).await {
        Ok(servants) => HttpResponse::Ok().json(servants),
        Err(_) => {
            let empty: Vec<Servant> = vec![];
            HttpResponse::Ok().json(empty)
        }
    }
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
            .service(servants)
    });
    server.bind("127.0.0.1:8000")?.run().await
}
