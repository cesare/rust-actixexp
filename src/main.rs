use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
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

#[get["/servants/sample"]]
async fn servants_sample() -> impl Responder {
    HttpResponse::Ok().json(Servant::new("Meltryllis", "alterego"))
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
            .service(servants_sample)
    });
    server.bind("127.0.0.1:8000")?.run().await
}
