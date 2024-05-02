extern crate diesel;

mod schema; 
mod handler;
mod model;
mod db;
mod route;
mod utils;
mod middleware;
#[cfg(test)]
mod test;

use actix_web::middleware::Logger; 
use actix_web::{web, App, HttpServer, HttpResponse, error};
use db::establish_connection_pool;
use db::DbPool;
use serde_json::json;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_cfg.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .route("/auth/heath_check", web::get().to(|| async {HttpResponse::Ok().body("OK") }))
                    .configure(route::config)
                    // .configure(routes::department::config)
                    // .configure(routes::user_department::config)   
            )
    })
    .bind(("0.0.0.0",8082))?
    .run()
    .await 
}


