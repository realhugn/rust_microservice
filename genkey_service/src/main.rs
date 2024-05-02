extern crate diesel;

mod db;
mod model;
mod schema;
mod utils;
mod rate_limit;
mod middleware;
mod handler;
mod route;

use std::sync::{Arc, Mutex};

use db::{establish_connection_pool,DbPool};
use actix_web::middleware::Logger; 
use actix_web::{web, App, HttpServer, HttpResponse, error};
use rate_limit::SlidingWindow;


#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        // use custom error handler
        .error_handler(|err, _| {
            error::InternalError::from_response(err, HttpResponse::BadRequest().body("FAIL").into()).into()
        });

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // create postgres connection pool 
    let pool: DbPool = establish_connection_pool(); 
    let sliding_window: Arc<Mutex<SlidingWindow>> = Arc::new(Mutex::new( SlidingWindow::new(10000, 2)));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_cfg.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .configure(route::config)
            )
            .service(
                web::scope("")
                .route("/health_check1", web::get().to(|| async {HttpResponse::Ok().body("OK from health check 1") }))
                .route("/health_check2", web::get().to(|| async {HttpResponse::Ok().body("OK from health check 2") }))
                .route("/health_check", web::get().to(|| async {HttpResponse::Ok().body("OK from health check") }))
                .wrap(middleware::SlidingWindowMiddleware{rate_limiter: sliding_window.clone()})
            )
    })
    .bind(("0.0.0.0",8080))?
    // .workers(1)
    .run()
    .await 
}


