use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{NoTls, Error};

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
struct Servant {
    name: String,
    class: String,
}

async fn list_servants() -> Result<Vec<Servant>, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    connection.await?;

    let rows = client.query("select name, class from servants", &[]).await?;
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
