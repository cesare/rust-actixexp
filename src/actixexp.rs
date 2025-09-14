use actix_cors::Cors;
use actix_web::http::header;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::{post, resource, scope, Data};
use app::middlewares::LoginRequired;
use env_logger::Env;

mod app;
use self::app::config::ApplicationConfig;
use self::app::context::Context;
use self::app::config::AppArgs;
use self::app::handlers::{self};

fn create_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_uri)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = AppArgs::new();
    let config = args.load_config().await?;
    let context = Context::initialize(&config)?;
    let bind_address = config.server.bind_address();
    let session_key = config.app.raw_session_key()?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        let session = SessionMiddleware::new(CookieSessionStore::default(), Key::from(&session_key));
        let cors = create_cors(&config);
        let login_required = LoginRequired::new();

        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(cors)
            .wrap(session)
            .app_data(Data::new(context.clone()))
            .service(handlers::root::index)
            .service(
                scope("/auth")
                    .configure(handlers::auth_service_config)
            )
            .service(
                scope("/servants")
                    .wrap(login_required)
                    .configure(handlers::servant_service_config)
            )
            .service(
                resource("/signout")
                    .route(post().to(handlers::sessions::signout))
            )
    });
    server.bind(bind_address)?.run().await?;

    Ok(())
}
