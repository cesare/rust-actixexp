use actix_web::{get, App, HttpResponse, HttpServer, Responder};
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

fn connection_parameters() -> String {
    let username = std::env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string());
    format!("host=localhost user={} password={} dbname=actixexp", username, password)
}

async fn list_servants() -> Result<Vec<Servant>, Error> {
    let parameters = connection_parameters();
    let (client, connection) = tokio_postgres::connect(&parameters, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection failed: {}", e);
        }
    });

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
async fn servants() -> impl Responder {
    match list_servants().await {
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
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(servants)
    });
    server.bind("127.0.0.1:8000")?.run().await
}
