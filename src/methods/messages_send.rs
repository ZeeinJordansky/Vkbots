use serde::Serialize;
use serde_json::Value;

use crate::{api::ApiClient, Result};

/// Типизированные параметры метода messages.send.
#[derive(Debug, Clone, Serialize)]
pub struct MessagesSendParams {
    pub peer_id: i64,
    pub random_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_parse_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_mentions: Option<bool>,
}

impl ApiClient {
    /// Отправляет сообщение с типизированными параметрами.
    pub async fn messages_send(&self, params: &MessagesSendParams) -> Result<i64> {
        self.call("messages.send", params).await
    }
}
