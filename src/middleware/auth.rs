use crate::{config::Config, infrastructure::auth::jwt::Claims};
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::{
    rc::Rc,
    task::{Context, Poll},
};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let config = req
                .app_data::<actix_web::web::Data<Config>>()
                .cloned()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("Config not found"))?;

            let authorization = req
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok());

            let Some(authorization) = authorization else {
                return Err(actix_web::error::ErrorUnauthorized(
                    "Missing Authorization header",
                ));
            };

            let Some(token) = authorization.strip_prefix("Bearer ") else {
                return Err(actix_web::error::ErrorUnauthorized(
                    "Invalid Authorization header",
                ));
            };

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

            req.extensions_mut().insert(token_data.claims.sub);

            service.call(req).await
        })
    }
}
