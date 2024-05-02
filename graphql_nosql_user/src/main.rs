use actix_web::{HttpServer, App, web, middleware};
use mongodb::Client;
use actix_cors::Cors;
use crate::handler::register;

mod config;
mod schemas;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .configure(register)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}