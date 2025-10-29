use std::time::Duration;

use axum::{body::Body, extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};
use rdkafka::{ClientConfig, producer::{FutureProducer, FutureRecord}};

use crate::config::structs::ApplicationState;

pub fn create_kafka_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "0.0.0.0:10000, 0.0.0.0:10001, 0.0.0.0:10002")
        .create()
        .expect("Producer creation error")
}

pub async fn send_user_view_content(
    State(state): State<ApplicationState>, 
    mut req: Request,
    next: Next
) -> Result<Response, impl IntoResponse> {

    let body = req.body().clone();
    let content_classifier = "music";
    let user_id = 12345;
    println!("Kafka send_user_view_content called with classifier: {}, user_id: {}", &content_classifier, &user_id);
    let producer = state.get_kafka_producer();
    let topic = String::from("user_view_content");
    let user_id_str = user_id.to_string();
    let payload = format!("{{\"user_id\": {}, \"content_classifier\": \"{}\"}}", user_id, content_classifier);
  

    tokio::spawn(async move {
        let produce_future = producer.send(
        FutureRecord::to(topic.as_str())
            .key(user_id_str.as_str())
            .payload(payload.as_str()), 
            Duration::from_secs(0));
        match produce_future.await {
            Ok(delivery) => {
                let partition = delivery.partition;
                let offset = delivery.offset;
                println!("메세지 발행 완료 topic: {}, partition: {}, offset: {}", topic, partition, offset)
            }
            Err((e, _)) => {
                eprintln!("메세지 발행 실패: {}", e)
            }
        }
    });

    let response = next.run(req).await;
    Ok::<Response<Body>, StatusCode>(response)


}

