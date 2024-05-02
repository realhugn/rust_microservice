use rdkafka::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct KafkaMessage {
    pub message_id: String,
    pub user_id : String,
    pub event_type: String,
    pub post_id: String,
    pub title: String,
    pub recipient: Vec<i32>
}
pub struct KafkaConsumer {
    pub consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> KafkaConsumer {
        let kafka_url = std::env::var("KAFKA_URL").expect("KAFKA_URL");
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url)
            .set("group.id", "default-group")
            .create()
            .expect("Consumer creation failed");
        consumer.subscribe(&["posts"]).expect("Can't subscribe to topic");
        KafkaConsumer { consumer }
    }
}
