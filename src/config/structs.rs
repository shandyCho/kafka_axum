use std::sync::Arc;

use rdkafka::producer::FutureProducer;

#[derive(Clone)]
pub struct ApplicationState {
    kafka_producer: Arc<FutureProducer>,
}

impl ApplicationState {
    pub fn new(kafka_producer: Arc<FutureProducer>) -> Self {
        ApplicationState { kafka_producer }
    }

    pub fn get_kafka_producer(&self) -> Arc<FutureProducer> {
        Arc::clone(&self.kafka_producer)
    }
}

