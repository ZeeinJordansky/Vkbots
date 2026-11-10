use serde::Deserialize;

use crate::{api::ApiClient, Result};

/// Данные подключения к серверу группового Long Poll.
#[derive(Debug, Clone, Deserialize)]
pub struct LongPollServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}

impl ApiClient {
    /// Запрашивает параметры Long Poll для сообщества.
    pub async fn groups_get_long_poll_server(&self, group_id: i64) -> Result<LongPollServer> {
        self.call("groups.getLongPollServer", &serde_json::json!({ "group_id": group_id }))
            .await
    }
}
