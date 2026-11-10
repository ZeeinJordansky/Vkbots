#![cfg(feature = "callback")]

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use axum::{
    extract::{Form, State},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use vkbots::VkBot;

#[derive(Clone, Default)]
struct TestState {
    calls: Arc<AtomicUsize>,
    params: Arc<Mutex<Vec<HashMap<String, String>>>>,
}

async fn api_method(
    State(state): State<TestState>,
    Form(params): Form<HashMap<String, String>>,
) -> Json<Value> {
    state.params.lock().unwrap().push(params);
    let call = state.calls.fetch_add(1, Ordering::SeqCst);

    if call == 0 {
        Json(json!({
            "error": {
                "error_code": 9,
                "error_msg": "Flood control",
                "request_params": []
            }
        }))
    } else {
        Json(json!({ "response": 77 }))
    }
}

#[tokio::test]
async fn api_retries_flood_control_and_keeps_auth_parameters() {
    let state = TestState::default();
    let app = Router::new()
        .route("/method/messages.test", post(api_method))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let bot = VkBot::builder("test-token")
        .api_base_url(format!("http://{address}/method"))
        .requests_per_second(100)
        .max_retries(1)
        .retry_delays(Duration::from_millis(1), Duration::from_millis(5))
        .build()
        .unwrap();

    let result: i64 = bot
        .api
        .call("messages.test", &json!({ "peer_id": 123 }))
        .await
        .unwrap();

    assert_eq!(result, 77);
    assert_eq!(state.calls.load(Ordering::SeqCst), 2);

    let requests = state.params.lock().unwrap();
    assert_eq!(requests[0].get("access_token").map(String::as_str), Some("test-token"));
    assert_eq!(requests[0].get("v").map(String::as_str), Some("5.199"));
    assert_eq!(requests[0].get("peer_id").map(String::as_str), Some("123"));
    assert_eq!(requests[0], requests[1]);
}

#[tokio::test]
async fn invalid_method_name_is_rejected_before_network_call() {
    let bot = VkBot::builder("test-token")
        .max_retries(0)
        .build()
        .unwrap();

    let error = bot
        .api
        .call::<Value, _>("messages.send/invalid", &json!({}))
        .await
        .unwrap_err();

    assert!(matches!(error, vkbots::Error::InvalidParameter(_)));
}
