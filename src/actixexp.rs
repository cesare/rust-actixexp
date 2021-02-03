use actix_web::{App, HttpServer};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use tokio_postgres::NoTls;

mod app;
use self::app::handlers::{self};

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool_config = create_pool_config();
    let pool = pool_config.create_pool(NoTls).unwrap();

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
