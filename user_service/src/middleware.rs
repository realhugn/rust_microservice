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
            verification_path: vec![("/v1/user/", "GET")],
            // noverification_path: vec![("/v1/user/", "POST")],
        }))
    }
}

pub struct JWTAuthHiMiddleware<S> {
    service: S,
    verification_path: Vec<(&'static str, &'static str)>,
   // noverification_path: Vec<(&'static str, &'static str)>,
}

impl<S, B> JWTAuthHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    fn is_need_verification(&self, path: &str, method: &str) -> bool {
        self.verification_path
            .iter()
            .any(|&vp| path.starts_with(vp.0) & method.eq(vp.1))
            // || !self
            //     .noverification_path
            //     .iter()
            //     .any(|&vp| path.starts_with(vp.0) & method.eq(vp.1))
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
        println!("{:?}", req.path());
        if self.is_need_verification(req.path(), req.method().as_str()) {
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

            let token_data = token_data.unwrap();
            req.extensions_mut().insert(token_data.claims);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}