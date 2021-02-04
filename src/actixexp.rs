use actix_web::{App, HttpServer};

mod app;
use crate::app::config::ActixexpConfig;
use self::app::handlers::{self};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = ActixexpConfig::new();
    let pool = config.create_pool();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::root::index)
            .service(handlers::servant::create)
            .service(handlers::servant::list)
            .service(handlers::servant::show)
            .service(handlers::servant::destroy)
    });
    server.bind("127.0.0.1:8000")?.run().await
}
