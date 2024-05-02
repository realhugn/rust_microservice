use std::time::Duration;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer
}

impl KafkaProducer {
    pub fn new() -> Self {
        let kafka_url = std::env::var("KAFKA_URL").expect("KAFKA_URL");
        let producer =  ClientConfig::new()
            .set("bootstrap.servers", kafka_url)
            .create().expect("Producer failed to create");
        KafkaProducer { producer }
    }

    pub async fn send_msg(&self, topic: &str, payload: &str) -> Result<(), String>{
        let record = FutureRecord::to(topic)
            .payload(payload)
            .key("");
        match self.producer.send(record, Duration::from_secs(0)).await  {
            Ok(_) => Ok(()),
            Err(e) =>Err(format!("{:?}", e))  
        }
    }
}