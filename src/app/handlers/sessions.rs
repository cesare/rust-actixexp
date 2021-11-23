use actix_session::Session;
use actix_web::HttpResponse;
use serde_json::json;

type Result<T, E = actix_web::Error> = std::result::Result<T, E>;

pub async fn signout(session: Session) -> Result<HttpResponse> {
    session.purge();
    let response = HttpResponse::Ok().json(json!({
        "status": "ok",
    }));
    Ok(response)
}
