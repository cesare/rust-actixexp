use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "servants")]
struct Servant {
    name: String,
    class: String,
}

impl Servant {
    fn new(name: impl Into<String>, class: impl Into<String>) -> Self {
        Servant {
            name: name.into(),
            class: class.into(),
        }
    }
}

#[get["/servants"]]
async fn servants() -> impl Responder {
    let servants: Vec<Servant> = vec![];
    HttpResponse::Ok().json(servants)
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
