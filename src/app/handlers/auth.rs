use actix_service::ServiceFactory;
use actix_session::Session;
use actix_web::{Error, HttpResponse, ResponseError, Scope};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::{Data, Query, get, scope};
use serde_json::json;

use crate::app::models::auth::{Authentication, AuthenticationError, AuthorizationRequest, CallbackParams};
use crate::app::config::ApplicationConfig;
use crate::app::Result as AppResult;

type Config = Data<ApplicationConfig>;

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl ServiceFactory<ServiceRequest, InitError = (), Error = Error, Response = ServiceResponse, Config = ()>> {
    scope("/auth")
        .app_data(Data::new(config.clone()))
        .route("", get().to(start))
        .route("/callback", get().to(callback))
}

async fn start(config: Config, session: Session) -> AppResult<HttpResponse> {
    let auth_request = AuthorizationRequest::new();
    session.insert("auth-state", &auth_request.state).unwrap(); // TODO: handle errors

    let response_json = json!({
        "client_id": &config.auth.client_id,
        "scope": "read:user",
        "state": &auth_request.state,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

type Params = Query<CallbackParams>;
type Result = std::result::Result<HttpResponse, AuthenticationError>;

impl ResponseError for AuthenticationError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

async fn callback(config: Config, session: Session, params: Params) -> Result {
    let key = "auth-state";
    let saved_state: Option<String> = session.get(key).unwrap(); // TODO: handle errors
    let _ = session.remove(key);

    let auth = Authentication::new(config.into_inner(), params.into_inner(), saved_state);
    let auth_result = auth.execute().await?;

    let response = HttpResponse::Ok().json(auth_result);
    Ok(response)
}
