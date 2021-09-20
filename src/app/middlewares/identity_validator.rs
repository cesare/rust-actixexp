use std::error::Error as StdError;

use actix_web::body::{AnyBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{ok, FutureExt as _, LocalBoxFuture, Ready};

pub struct IdentityValidator;

impl<S, B> Transform<S, ServiceRequest> for IdentityValidator
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
    type Transform = IdentityValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(IdentityValidatorMiddleware {
            service: service,
        })
    }
}

pub struct IdentityValidatorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for IdentityValidatorMiddleware<S>
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
