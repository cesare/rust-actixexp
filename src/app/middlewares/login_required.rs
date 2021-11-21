use std::error::Error as StdError;

use actix_web::body::{AnyBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{ok, FutureExt as _, LocalBoxFuture, Ready};

use crate::app::config::ApplicationConfig;

pub struct LoginRequired {
}

impl LoginRequired {
    #[allow(dead_code)]
    pub fn new(_config: &ApplicationConfig) -> Self {
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
    B::Error: StdError,
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
    B::Error: StdError,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        async move {
            let res = fut.await?;
            Ok(res.map_body(|_, body| AnyBody::from_message(body)))
        }
        .boxed_local()
    }
}
