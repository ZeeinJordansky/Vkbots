use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::Message;

/// Событие Callback API или Long Poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type", alias = "event_type")]
    pub event_type: String,
    #[serde(default)]
    pub group_id: i64,
    #[serde(default)]
    pub object: Value,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub v: Option<String>,
}

impl Event {
    /// Возвращает сообщение, если событие связано с сообщением.
    pub fn message(&self) -> Option<Message> {
        let value = self
            .object
            .get("message")
            .or_else(|| {
                if self.object.get("peer_id").is_some() || self.object.get("text").is_some() {
                    Some(&self.object)
                } else {
                    None
                }
            })?;

        serde_json::from_value(value.clone()).ok()
    }

    pub(crate) fn from_long_poll(group_id: i64, update: Value) -> Self {
        let code = update
            .as_array()
            .and_then(|values| values.first())
            .and_then(Value::as_i64)
            .unwrap_or_default();

        // В классическом формате Long Poll входящее сообщение имеет код 4.
        let (event_type, object) = if code == 4 {
            let values = update.as_array();
            let message = json!({
                "id": values.and_then(|v| v.get(1)).and_then(Value::as_i64),
                "peer_id": values.and_then(|v| v.get(3)).and_then(Value::as_i64),
                "date": values.and_then(|v| v.get(4)).and_then(Value::as_i64),
                "text": values.and_then(|v| v.get(5)).and_then(Value::as_str),
            });
            ("message_new".to_owned(), json!({ "message": message, "raw": update }))
        } else {
            (format!("longpoll_{code}"), json!({ "raw": update }))
        };

        Self {
            event_type,
            group_id,
            object,
            event_id: None,
            secret: None,
            v: None,
        }
    }
}
