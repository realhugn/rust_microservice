use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{middleware::Logger, HttpServer, App, web};
use diesel::{r2d2::{ConnectionManager, self}, PgConnection};
use handler::{create, index, heath_check};
use kafka::{KafkaConsumer, KafkaMessage};
use log::warn;
use rdkafka::{Message, consumer::{CommitMode, Consumer}};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod kafka;
mod schema;
mod model;
mod handler;
mod middlewares;
mod utils;
mod recipient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let consumer = KafkaConsumer::new().consumer;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let pool_clone = pool.clone();
    let mut total_time = 0;
    let mut count = 0; 
    actix_web::rt::spawn(async move {
        loop {
            match consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    let post_payload : KafkaMessage =match serde_json::from_str(payload) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            continue;
                        }
                    };
                    let mut conn = pool_clone.get().unwrap();
                    let _ = create(post_payload, &mut conn);
                    
                    consumer.commit_message(&m, CommitMode::Async).expect("Error Committing Consumed Message");
                    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    total_time += end - start;
                    count += 1;
                    if count == 10000 {
                        println!("{}", total_time);
                    }
                }
            };
        };
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default()) 
            .wrap(middlewares::JWTAuth)
            .service(
                web::scope("/v1/notification")
                    .service(index)
                    .service(heath_check)
            )
    })
    .bind(("0.0.0.0",8083))?
    .run()
    .await 
}