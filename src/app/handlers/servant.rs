use actix_web::web::{delete, get, post, Data, Json, Path, ServiceConfig};
use actix_web::HttpResponse;
use serde::Deserialize;
use serde_json::json;

use crate::app::context::Context;
use crate::app::models::servant::{ServantDeletion, ServantFetching, ServantListing, ServantRegistration};

type Ctx = Data<Context>;
type Result<T, E = actix_web::Error> = std::result::Result<T, E>;

pub fn servant_service_config(config: &mut ServiceConfig) {
    config
        .route("", get().to(list))
        .route("", post().to(create))
        .route("/{id}", get().to(show))
        .route("/{id}", delete().to(destroy));
}

#[derive(Deserialize)]
struct CreateServantRequest {
    name: String,
    class_name: String,
}

async fn create(context: Ctx, request: Json<CreateServantRequest>) -> Result<HttpResponse> {
    let registration = ServantRegistration::new(&context, &request.name, &request.class_name);
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
