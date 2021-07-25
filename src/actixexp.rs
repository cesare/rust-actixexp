use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;

mod app;
use crate::app::config::ActixexpConfig;
use self::app::handlers::{self};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = ActixexpConfig::new();
    let pool = config.create_pool();

    let bind_address = config.bind_address();
    let app_config = config.app_config;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .app_data(Data::new(pool.clone()))
            .service(handlers::root::index)
            .service(handlers::servant::create_scope(&app_config))
    });
    server.bind(bind_address)?.run().await
}
