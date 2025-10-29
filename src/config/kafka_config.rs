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

pub async fn produce_user_traffic_log(
    State(state): State<ApplicationState>, 
    mut req: Request,
    next: Next
) -> Result<Response, impl IntoResponse> {

    let uri = req.uri();
    tracing::info!("send_user_view_content middleware called with body: {:?}", uri);
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


// 이벤트 드라이븐 방식으로 구현하여서 이벤트 발행 -> 카프카에 전송하는 방식으로도 사용하는듯?
// 로드밸런싱과 보내는 쪽에서의 트래픽 과부하도 고려해야할 듯?
// 스키마 레지스트리는 프로듀서와 컨슈머가 동일한 데이터 타입을 사용할 수 있도록 보장하고, 데이터의 압축의 효율화를 꾀할 수 있다
// 스키마 레지스트리는 데이터 타입의 변경이 필요할 때도 버전별로 데이터 형식을 관리할 수 있기 때문에 유용하다
// -> 임의로 데이터 형식 중 일부가 삭제되더라도 버저닝이 되기 때문에 이전 버전의 데이터 형식을 사용하는 컨슈머가 문제없이 작동할 수 있다
// 스키마 레지스트리는 카프카와는 또 다른 별도의 서비스로 운영된다 -> 컨플루언트에서 만든 도커 이미지가 있음
// 스키마 레지스트리 사용을 위해서 Backward Forward 설정이 필요함
// 토픽의 사용처는 점점 많아질수록 사용하는 곳이 어딘지 알 수 없는 경우가 많음
// 토픽에 의한 의존성이 생기는 케이스도 있음
    // 테라프? 같은 것으로 리소스 형식을 지정하는 경우도 있음?
// 11/12 스터디 계획
// 책 6, 7장 읽고 정리해오기

