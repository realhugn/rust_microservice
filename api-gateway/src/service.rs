use std::sync::{Mutex, Arc};

use hyper::header::HeaderValue;
use hyper::http::request::Parts;
use hyper::{Body, Client, Request, Response};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use tracing::{info, warn};

use crate::config::{GatewayConfig, ServiceConfig, DbPool};
use crate::rate_limit::SlidingWindow;
use crate::utils::{verify_jwt_token, authorize_request_group};

// Handle incoming requests
pub async fn handle_request(
    req: Request<Body>,
    config: GatewayConfig,
    pool: DbPool, 
    limiter : Arc<Mutex<SlidingWindow>>
) -> Result<Response<Body>, hyper::Error> {
    // Get the requested path
    let path = req.uri().path();

    // Log that an incoming request was received
    info!("Incoming request for path: {}", path);

    // Check if the requested path is the health-check endpoint
    if path == "/health-check" {
        return health_check();
    }

    // Get the service configuration for the requested path
    let service_config = match get_service_config(path.clone(), &config.services) {
        Some(service_config) => service_config,
        None => {
            // If no service configuration exists for the requested path, return a 404 response
            warn!("Path not found: {}", path);
            return not_found();
        }
    };

    // Check if the requested service requires authentication
    let auth_token = if service_config.authentication_required.unwrap_or(true) {
        // If so, authorize the user by sending a request to the authorization API
        // match authorize_user(&req.headers()).await {
        //     Some(header) => {
        //         let token = &header[7..];
        //         let token_claims = match verify_jwt_token(&token) {
        //             Ok(data) => {
        //                 data.claims
        //             },
        //             Err(_) => {
        //                 return unauthorized();
        //             }
        //         } ;
        //         let mut conn = match pool.get() {
        //             Ok(conn) => {
        //                 conn
        //             },
        //             Err(_) => return service_unavailable("Db error"),
        //         };

        //         let mut user_bucket = match new_bucket_for_uid(token_claims.sub, &mut conn) {
        //             Ok(bucket) => {
        //                 bucket
        //             }, 
        //             Err(_) => {
        //                 return service_unavailable("Db error");
        //             } 
        //         };

        //         let _ = match consume(&mut user_bucket,  &mut conn) {
        //             Ok(rs) => {
        //                 if !rs {
        //                     return service_unavailable("Too many request");
        //                 }
        //             }, 
        //             Err(_) => {
        //                 return service_unavailable("Db error");
        //             } 
        //         };
                
        //         header
        //     },
        //     None => {
        //         // If there is an error connecting to the authorization API, return a 503 response
        //         warn!("Error");
        //         return unauthorized();
        //     }
        // }

        match authorize_user(&req.headers()).await {
            Some(header) => {
                let token = &header[7..];
                let token_claims = match verify_jwt_token(&token) {
                    Ok(data) => {
                        data.claims
                    },
                    Err(_) => {
                        warn!("Error in API Gateway");
                        return unauthorized();
                    }
                };
                
                let mut limiter = limiter.lock().unwrap();
                if !limiter.allow(token_claims.sub) {
                    return toomanyrequest();
                }
                if !authorize_request_group(pool, req.uri().path().into(), token_claims.sub) {
                    return unauthorized()
                }
                header
            },
            None => {
                // If there is an error connecting to the authorization API, return a 503 response
                warn!("Error");
                return unauthorized();
            }
        }
    } else {
        info!("No Auth Header");
        String::new()
    };



    // Build the downstream request
    let (parts, body) = req.into_parts();
    let downstream_req = build_downstream_request(parts, body, service_config, auth_token).await?;

    // Forward the request to the requested service
    match forward_request(downstream_req).await {
        Ok(res) => {
            // If the request is successful, log that it was forwarded and return the response
            info!("Forwarded request successfully");
            Ok(res)
        }
        Err(_) => {
            // If there is an error connecting to the requested service, return a 503 response
            warn!("Failed to connect to downstream service");
            service_unavailable("Failed to connect to downstream service")
        }
    }
}

// Get the service configuration for the requested path
fn get_service_config<'a>(path: &str, services: &'a [ServiceConfig]) -> Option<&'a ServiceConfig> {
    services.iter().find(|c| c.path.is_match(path))
}

// Authorize the user by sending a request to the authorization API
async fn authorize_user(headers: &HeaderMap) -> Option<String> {
    let auth_header_value = match headers.get(AUTHORIZATION) {
        Some(value) => Some(value.to_str().unwrap_or_default().to_string()),
        None => {
            // If the authorization header is missing, log a warning and return an empty string
            warn!("Authorization header not found");
            None 
        }
    };

    auth_header_value
}

// Build the downstream request
async fn build_downstream_request(
    parts: Parts,
    body: Body,
    service_config: &ServiceConfig,
    auth_token: String,
) -> Result<Request<Body>, hyper::Error> {
    let req = Request::from_parts(parts, body);
    let query = match req.uri().query() {
        Some (q) => format!("?{}", q),
        None => "".to_string()
    };
    let uri = format!(
        "{}:{}{}{}",
        service_config.target_service,
        service_config.target_port,
        req.uri().path(),
        query
    );

    let mut downstream_req_builder = Request::builder()
        .uri(uri)
        .method(req.method())
        .version(req.version());

    *downstream_req_builder.headers_mut().unwrap() = req.headers().clone();

    downstream_req_builder
        .headers_mut()
        .unwrap()
        .insert("Authorization", HeaderValue::from_str(&auth_token).unwrap());
    
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    // Log that the downstream request is being built and return the completed request
    info!("Building downstream request");
    let downstream_req = downstream_req_builder.body(Body::from(body_bytes));
    Ok(downstream_req.unwrap())
}

// Forward the request to the requested service
async fn forward_request(req: Request<Body>) -> Result<Response<Body>, ()> {
    match Client::new().request(req).await {
        Ok(res) => {
            // If the request is successful, log that it was successful and return the response
            info!("Request forwarded successfully");
            Ok(res)
        }
        Err(_) => {
            // If there is an error connecting to the requested service, return an error
            warn!("Failed to forward request");
            Err(())
        }
    }
}

// Return a 200 response for the health check
fn health_check() -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("OK"));
    info!("Responding with 200 OK for health check");
    Ok(response)
}

// Return a 404 response
fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    warn!("Responding with 404 Not Found");
    Ok(response)
}

// Return a 503 response with a reason
fn service_unavailable<T>(reason: T) -> Result<Response<Body>, hyper::Error>
where
    T: Into<Body>,
{
    let mut response = Response::new(reason.into());
    *response.status_mut() = hyper::StatusCode::SERVICE_UNAVAILABLE;
    warn!("Responding with 503 Service Unavailable");
    Ok(response)
}

fn unauthorized() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("403 Unauthorized"));
    *response.status_mut() = hyper::StatusCode::UNAUTHORIZED;
    warn!("Responding with 403 Unauthorized");
    Ok(response)
}

fn toomanyrequest() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("429 Too Many Requests"));
    *response.status_mut() = hyper::StatusCode::TOO_MANY_REQUESTS;
    warn!("Responding with 429 Too Many Requests");
    Ok(response)
}