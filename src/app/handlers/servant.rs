use actix_cors::Cors;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::web::{delete, get, post, scope, Data, Json, Path};
use actix_web::{HttpResponse, Scope};
use serde::Deserialize;
use serde_json::json;

use crate::app::config::ApplicationConfig;
use crate::app::context::Context;
use crate::app::middlewares::IdentityValidator;
use crate::app::models::servant::{ServantDeletion, ServantFetching, ServantListing, ServantRegistration};

type Ctx = Data<Context>;
type Result<T, E = actix_web::Error> = std::result::Result<T, E>;

fn create_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_uri)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl actix_service::ServiceFactory<ServiceRequest, InitError = (), Error = actix_web::Error, Response = ServiceResponse, Config = ()>> {
    let cors = create_cors(config);
    let identity_validator = IdentityValidator::new(config);
    scope("/servants")
        .wrap(cors)
        .wrap(identity_validator)
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy))
}

#[derive(Deserialize)]
struct CreateServantRequest {
    name: String,
    class_name: String,
}

async fn create(context: Ctx, form: Json<CreateServantRequest>) -> Result<HttpResponse> {
    let registration = ServantRegistration::new(&context, &form.name, &form.class_name);
    let servant = registration.execute().await?;
    let response = HttpResponse::Created().json(servant);
    Ok(response)
}

async fn list(context: Ctx) -> Result<HttpResponse> {
    let listing = ServantListing::new(&context);
    let servants = listing.execute().await?;
    let response_json = json!({
        "servants": servants,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn show(context: Ctx, path: Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let fetching = ServantFetching::new(&context, id);
    let servant = fetching.execute().await?;
    let response = HttpResponse::Ok().json(servant);
    Ok(response)
}

async fn destroy(context: Ctx, path: Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let deletion = ServantDeletion::new(&context, id);
    let servant = deletion.execute().await?;
    let response = HttpResponse::Ok().json(servant);
    Ok(response)
}
