// main.rs는 어플리케이션의 진입점이며 라우터 설정등을 진행할 수 있다
// main.rs에서 다른 모듈의 요소를 사용하고자 할 때는 해당 모듈에 대해서 mod 키워드를 사용해서 선언해야한다.
// 크레이트 루트 (crate root) 는 러스트 컴파일러가 컴파일을 시작하는 소스 파일이고, 크레이트의 루트 모듈을 구성합니다.
mod dashboard;
mod config;

use std::sync::Arc;

use axum::routing::{get};
use axum::{Json, Router, middleware};
use serde::{Serialize};
use tower::ServiceBuilder;

use crate::dashboard::dashboard_handler::load_dashboard;
use crate::config::kafka_config::create_kafka_producer;
use crate::config::structs::ApplicationState;

// JSON 직렬화를 위한 트레이트를 자동으로 구현
#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // // 로깅 구독자 초기화 및 시작

    let kafka_producer = create_kafka_producer("localhost:9092");
    let application_state = ApplicationState::new(Arc::new(kafka_producer));

    let service_layer = ServiceBuilder::new()
    .layer(config::logging_config::config2::logging_setup2())
    .layer(middleware::from_fn_with_state(application_state.clone(), config::kafka_config::send_user_view_content));
    // 서버 IP 및 포트 정의
    let addr = "0.0.0.0:3000";
    // 라우터 정의
    let router = Router::new()
    .route("/", get(|| async {" Hello, World!"}))
    .route("/api/v1/hello", get(hello))
    .route("/api/v1/dashboard", get(load_dashboard))
    .layer(service_layer)
    .with_state(application_state);



    // 서버 TCP 포트 리스닝을 통한 서버 구동
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}

async fn hello() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}