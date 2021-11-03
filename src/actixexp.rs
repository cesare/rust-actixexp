use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use app::context::Context;
use env_logger::Env;

mod app;
use crate::app::config::AppArgs;
use self::app::handlers::{self};

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = AppArgs::new();
    let config = args.load_config().await?;
    let context = Context::initialize(&config)?;
    let bind_address = config.server.bind_address();
    let pool = config.database.create_pool()?;
    let session_key = config.app.raw_session_key()?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        let session = CookieSession::signed(&session_key).secure(false);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(session)
            .app_data(Data::new(context.clone()))
            .app_data(Data::new(pool.clone()))
            .service(handlers::root::index)
            .service(handlers::servant::create_scope(&config))
            .service(handlers::auth::create_scope(&config))
    });
    server.bind(bind_address)?.run().await?;

    Ok(())
}
