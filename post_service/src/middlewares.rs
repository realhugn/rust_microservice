use actix_web::{error::ErrorUnauthorized, Error, HttpMessage};
use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;

pub struct JWTAuth;
impl<S, B> Transform<S, ServiceRequest> for JWTAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTAuthHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthHiMiddleware {
            service,
            allow_role: vec![0, 1],
        }))
    }
}

pub struct JWTAuthHiMiddleware<S> {
    service: S,
    allow_role: Vec<i32>,
}

impl<S, B> JWTAuthHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    fn is_need_verification(&self, role: i32) -> bool {
        self.allow_role
            .iter()
            .any(|&vp| role.eq(&vp))
    }
}

impl<S, B> Service<ServiceRequest> for JWTAuthHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authorization = req.headers().get("Authorization");
        if authorization.is_none() {
            return Box::pin(async { Err(ErrorUnauthorized("err")) });
        }

        let authorization = authorization.unwrap().to_str();
        if authorization.is_err() {
            return Box::pin(async { Err(ErrorUnauthorized("err")) });
        }

        let authorization = authorization.unwrap();
        let token = &authorization[7..]; // 'Bearer ' + token

        let token_data = crate::utils::verify_jwt_token(token);

        if let Err(err) = token_data {
            return Box::pin(async { Err(ErrorUnauthorized(err)) });
        }

        let claims = token_data.unwrap();
        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        }
    )}
}