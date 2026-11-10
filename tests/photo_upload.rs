#![cfg(feature = "callback")]

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use axum::{
    extract::{Multipart, State, Form},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use vkbots::VkBot;

#[derive(Clone)]
struct UploadState {
    upload_url: String,
    upload_count: Arc<AtomicUsize>,
}

async fn get_upload_server(
    State(state): State<UploadState>,
    Form(_params): Form<HashMap<String, String>>,
) -> Json<Value> {
    Json(json!({
        "response": {
            "upload_url": state.upload_url,
            "album_id": 1,
            "user_id": -100
        }
    }))
}

async fn upload_photo(
    State(state): State<UploadState>,
    mut multipart: Multipart,
) -> Json<Value> {
    let mut found_photo = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("photo") {
            let bytes = field.bytes().await.unwrap();
            found_photo = !bytes.is_empty();
        }
    }

    assert!(found_photo);
    state.upload_count.fetch_add(1, Ordering::SeqCst);

    Json(json!({
        "server": 12,
        "photo": "[{\"photo\":\"test\"}]",
        "hash": "upload-hash"
    }))
}

async fn save_photo(
    Form(params): Form<HashMap<String, String>>,
) -> Json<Value> {
    assert_eq!(params.get("server").map(String::as_str), Some("12"));
    assert_eq!(params.get("hash").map(String::as_str), Some("upload-hash"));

    Json(json!({
        "response": [{
            "id": 77,
            "owner_id": -100,
            "access_key": "photo-key"
        }]
    }))
}

#[tokio::test]
async fn local_photo_upload_runs_the_vk_upload_chain() {
    let upload_count = Arc::new(AtomicUsize::new(0));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let state = UploadState {
        upload_url: format!("http://{address}/upload"),
        upload_count: upload_count.clone(),
    };

    let app = Router::new()
        .route("/method/photos.getMessagesUploadServer", post(get_upload_server))
        .route("/upload", post(upload_photo))
        .route("/method/photos.saveMessagesPhoto", post(save_photo))
        .with_state(state);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let bot = VkBot::builder("test-token")
        .api_base_url(format!("http://{address}/method"))
        .requests_per_second(100)
        .max_retries(0)
        .build()
        .unwrap();

    let path = std::env::temp_dir().join(format!("vkbots-photo-{}.png", std::process::id()));
    tokio::fs::write(&path, b"test image bytes").await.unwrap();

    let photo = bot.api.upload_message_photo(2000000001, &path).await.unwrap();
    let _ = tokio::fs::remove_file(&path).await;

    assert_eq!(photo.id, 77);
    assert_eq!(photo.owner_id, -100);
    assert_eq!(photo.attachment(), "photo-100_77_photo-key");
    assert_eq!(upload_count.load(Ordering::SeqCst), 1);
}
