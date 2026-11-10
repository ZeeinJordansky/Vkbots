use std::{future::Future, net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use futures_util::future::BoxFuture;
use serde_json::Value;
use tokio::net::TcpListener;

use crate::{Error, Event, Result};

/// Параметры Callback API сообщества.
#[derive(Clone)]
pub struct CallbackConfig {
    pub group_id: i64,
    pub secret: Option<String>,
    pub confirmation_key: Option<String>,
}

impl CallbackConfig {
    /// Создаёт конфигурацию для указанного сообщества.
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            secret: None,
            confirmation_key: None,
        }
    }

    /// Устанавливает секретный ключ Callback API.
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    /// Устанавливает строку подтверждения адреса сервера.
    pub fn confirmation_key(mut self, key: impl Into<String>) -> Self {
        self.confirmation_key = Some(key.into());
        self
    }
}

type EventHandler = Arc<dyn Fn(Event) -> BoxFuture<'static, Result<()>> + Send + Sync>;

#[derive(Clone)]
struct CallbackState {
    config: CallbackConfig,
    handler: EventHandler,
}

/// HTTP-сервер для получения событий Callback API.
pub struct CallbackServer {
    state: CallbackState,
}

impl CallbackServer {
    /// Создаёт сервер Callback API.
    pub fn new(config: CallbackConfig) -> Self {
        Self {
            state: CallbackState {
                config,
                handler: Arc::new(|_| Box::pin(async { Ok(()) })),
            },
        }
    }

    /// Назначает обработчик события без возвращаемой ошибки.
    pub fn on_event<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.state.handler = Arc::new(move |event| {
            let future = handler(event);
            Box::pin(async move {
                future.await;
                Ok(())
            })
        });
        self
    }

    /// Назначает обработчик, который может вернуть ошибку.
    ///
    /// При ошибке сервер ответит HTTP 500, чтобы VK повторно доставил событие.
    pub fn on_event_result<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Event) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.state.handler = Arc::new(move |event| Box::pin(handler(event)));
        self
    }

    /// Возвращает маршрутизатор Axum для встраивания в существующее приложение.
    pub fn router(&self) -> Router {
        Router::new()
            .route("/", post(handle_callback))
            .with_state(self.state.clone())
    }

    /// Запускает HTTP-сервер по указанному адресу.
    pub async fn serve(self, address: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(address).await?;
        axum::serve(listener, self.router())
            .await
            .map_err(Error::Io)
    }
}

async fn handle_callback(
    State(state): State<CallbackState>,
    Json(payload): Json<Value>,
) -> Response {
    let event: Event = match serde_json::from_value(payload) {
        Ok(event) => event,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid event").into_response(),
    };

    if event.group_id != state.config.group_id {
        return (StatusCode::FORBIDDEN, "group_id mismatch").into_response();
    }

    if let Some(expected) = &state.config.secret {
        if event.secret.as_deref() != Some(expected.as_str()) {
            return (StatusCode::FORBIDDEN, "secret mismatch").into_response();
        }
    }

    if event.event_type == "confirmation" {
        return match &state.config.confirmation_key {
            Some(key) => (StatusCode::OK, key.clone()).into_response(),
            None => (StatusCode::INTERNAL_SERVER_ERROR, "confirmation key is missing")
                .into_response(),
        };
    }

    match (state.handler)(event).await {
        Ok(()) => (StatusCode::OK, "ok").into_response(),
        Err(error) => {
            eprintln!("Ошибка обработчика Callback API: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "handler failed").into_response()
        }
    }
}
