use mongodb::Client;

pub async fn establish_mongodb_connection() -> Client {
    let uri = std::env::var("MONGO_URL").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client_options = mongodb::options::ClientOptions::parse(uri).await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    client
}
