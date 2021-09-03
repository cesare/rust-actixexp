use actix_http::Method;
use actix_service::ServiceFactory;
use actix_session::Session;
use actix_web::{Error, HttpResponse, ResponseError, Route, Scope};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use actix_web::web::{Data, Form, post, scope};
use deadpool_postgres::{Pool};
use serde_json::json;

use crate::app::models::auth::{Authentication, AuthenticationError, AuthorizationRequest, CallbackParams};
use crate::app::config::ApplicationConfig;

type Result = std::result::Result<HttpResponse, AuthenticationError>;
type Config = Data<ApplicationConfig>;

impl ResponseError for AuthenticationError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthenticationError::StateMissing | AuthenticationError::StateNotMatch => {
                HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "reason": self.to_string(),
                }))
            }
            _ => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "internal server error",
                    "reason": self.to_string(),
                }))
            }
        }
    }
}

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl ServiceFactory<ServiceRequest, InitError = (), Error = Error, Response = ServiceResponse, Config = ()>> {
    let cors_headers = DefaultHeaders::new()
        .header("Access-Control-Allow-Origin", &config.frontend.base_uri)
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Credentials", "true");
    scope("/auth")
        .app_data(Data::new(config.clone()))
        .wrap(cors_headers)
        .route("", post().to(start))
        .route("/callback", post().to(callback))
        .route("", Route::new().method(Method::OPTIONS).to(options))
        .route("/callback", Route::new().method(Method::OPTIONS).to(options))
}

async fn options() -> Result {
    let response = HttpResponse::NoContent().finish();
    Ok(response)
}

async fn start(config: Config, session: Session) -> Result {
    let auth_request = AuthorizationRequest::new();
    session.insert("auth-state", &auth_request.state)
        .or(Err(AuthenticationError::StateSavingFailed))?;

    let response_json = json!({
        "client_id": &config.auth.client_id,
        "scope": "read:user",
        "state": &auth_request.state,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

type Params = Form<CallbackParams>;
type DbPool = Data<Pool>;

async fn callback(config: Config, _pool: DbPool, session: Session, params: Params) -> Result {
    let key = "auth-state";
    let saved_state: Option<String> =
        session.get(key).or(Err(AuthenticationError::StateLoadingFailed))?;
    let _ = session.remove(key);

    let auth = Authentication::new(config.into_inner(), params.into_inner(), saved_state);
    let auth_result = auth.execute().await?;

    let response = HttpResponse::Ok().json(auth_result);
    Ok(response)
}
