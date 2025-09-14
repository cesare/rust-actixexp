use actix_session::{SessionExt};
use actix_web::{Error, HttpResponse};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{ok, FutureExt as _, LocalBoxFuture, Ready};
use serde_json::json;

pub struct LoginRequired {
}

impl LoginRequired {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoginRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type InitError = ();
    type Transform = LoginRequiredMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoginRequiredMiddleware {
            service: service,
        })
    }
}

pub struct LoginRequiredMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoginRequiredMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let validator = LoginValidator::new(&req);
        let login_status = validator.execute();
        if let Err(res) = login_status {
            let response = req.into_response(res);
            return Box::pin(ok(response))
        }

        let fut = self.service.call(req);
        async move {
            let res = fut.await?;
            Ok(res.map_body(|_, body| BoxBody::new(body)))
        }
        .boxed_local()
    }
}

struct LoginValidator<'a> {
    request: &'a ServiceRequest,
}

impl<'a> LoginValidator<'a> {
    fn new(request: &'a ServiceRequest) -> Self {
        Self {
            request: request,
        }
    }

    fn execute(&self) -> std::result::Result<(), HttpResponse> {
        let session = self.request.get_session();
        let value = session.get::<String>("id");
        match value {
            Ok(Some(_id)) => Ok(()),
            Ok(None) => {
                let response = HttpResponse::Unauthorized().json(json!({
                    "error": "login required",
                }));
                Err(response)
            },
            Err(_error) => {
                let response = HttpResponse::InternalServerError().json(json!({
                    "error": "internal server error",
                }));
                Err(response)
            }
        }
    }
}
