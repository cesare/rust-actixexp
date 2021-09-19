use std::error::Error as StdError;

use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{LocalBoxFuture, Ready};

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
        todo!()
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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        todo!()
    }
}
