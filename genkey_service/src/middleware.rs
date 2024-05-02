use actix_web::{error::{ErrorUnauthorized, ErrorTooManyRequests, ErrorInternalServerError}, Error};
use regex::Regex;
use std::{future::{ready, Ready}, sync::{Mutex, Arc}, rc::Rc};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;

use crate::rate_limit::SlidingWindow;

pub struct SlidingWindowMiddleware {
    pub rate_limiter: Arc<Mutex<SlidingWindow>>,
}

impl<S, B> Transform<S, ServiceRequest> for SlidingWindowMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SlidingWindowMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SlidingWindowMiddlewareService {
            service:  Rc::new(service) ,
            rate_limiter: self.rate_limiter.clone(),
        }))
    }
}

pub struct SlidingWindowMiddlewareService<S> {
    service: Rc<S>,
    rate_limiter:  Arc<Mutex<SlidingWindow>>
}

impl<S, B> SlidingWindowMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    // fn is_need_verification(&self, role: i32) -> bool {
    //     self.allow_role
    //         .iter()
    //         .any(|&vp| role.eq(&vp))
    //         // || !self
    //         //     .noverification_role
    //         //     .iter()
    //         //     .any(|&vp| path.starts_with(vp.0) & method.eq(vp.1))
    // }
}

impl<S, B> Service<ServiceRequest> for SlidingWindowMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let rate_limiter = self.rate_limiter.clone();
        let svc = self.service.clone();
        Box::pin(async move {
            let authorization = req.headers().get("Authorization");
            if authorization.is_none() {
                return Err(ErrorUnauthorized("err"));
            }

            let authorization = authorization.unwrap().to_str();
            if authorization.is_err() {
                return Err(ErrorUnauthorized("err"));
            }

            let authorization = authorization.unwrap();
            let token = &authorization[7..]; // 'Bearer ' + token

            let token_data = crate::utils::verify_jwt_token(token.to_string());

            if let Err(err) = token_data {
                return Err(ErrorUnauthorized(err));
            }
            println!("{:?}", req.uri().path());
            if !token_data.unwrap().allow.iter().any(|endpoint| {
                let pattern = format!("{}$",endpoint);
                let re = Regex::new(&pattern).unwrap();
                let Some(caps) = re.captures(req.uri().path()) else {
                    println!("no match");
                    return false;
                };
                return true;
            }) {
                return Err(ErrorUnauthorized("Not Allow"))
            }
            
            match rate_limiter.lock() {
                Ok(mut rate_limiter) => {
                    if !rate_limiter.allow(token.to_string()) {
                        return Err(ErrorTooManyRequests("Too many requests"));
                    }
                }, 
                Err(_) => return Err(ErrorInternalServerError("Error")),
            }

            // if !rate_limiter.lock().unwrap().allow(token.to_string()) {
            //     return Err(ErrorTooManyRequests("Too many requests"));
            // }

            let fut = svc.call(req);
            let res = fut.await.unwrap();
            Ok(res)
        })
    }
}
