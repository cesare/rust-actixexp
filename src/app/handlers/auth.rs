use actix_cors::Cors;
use actix_service::ServiceFactory;
use actix_session::Session;
use actix_web::{Error, HttpResponse, ResponseError, Scope};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::web::{Data, Form, delete, post, scope};
use serde_json::json;

use crate::app::context::Context;
use crate::app::models::Identity;
use crate::app::models::auth::{Authentication, AuthenticationError, AuthorizationRequest, CallbackParams};
use crate::app::config::ApplicationConfig;

type Result = std::result::Result<HttpResponse, AuthenticationError>;
type Ctx = Data<Context>;

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

fn create_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_uri)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

pub fn create_scope(config: &ApplicationConfig) -> Scope<impl ServiceFactory<ServiceRequest, InitError = (), Error = Error, Response = ServiceResponse, Config = ()>> {
    let cors = create_cors(config);
    scope("/auth")
        .app_data(Data::new(config.clone()))
        .wrap(cors)
        .route("", post().to(start))
        .route("/callback", post().to(callback))
        .route("/session", delete().to(signout))
}

async fn start(context: Ctx, session: Session) -> Result {
    let config = &context.config;

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

async fn callback(context: Ctx, session: Session, params: Params) -> Result {
    let key = "auth-state";
    let saved_state: Option<String> =
        session.get(key).or(Err(AuthenticationError::StateLoadingFailed))?;
    let _ = session.remove(key);

    let auth = Authentication::new(&context, params.into_inner(), saved_state);
    let auth_result = auth.execute().await?;

    session.clear();
    session.renew();
    set_identity_to_session(&session, &auth_result.identity)?;

    let json = json!({
        "identifier": auth_result.identity.id,
        "name": auth_result.name,
    });
    let response = HttpResponse::Ok().json(json);
    Ok(response)
}

fn set_identity_to_session(session: &Session, identity: &Identity) -> std::result::Result<(), AuthenticationError> {
    session.insert("id", &identity.id)
        .or(Err(AuthenticationError::TokenSavingFailed))?;
    Ok(())
}

async fn signout(session: Session) -> Result {
    session.purge();

    let json = json!({
        "result": "ok",
    });
    let response = HttpResponse::Ok().json(json);
    Ok(response)
}
