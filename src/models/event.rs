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
        let values = update.as_array();
        let raw_values = values.map(Vec::as_slice).unwrap_or_default();
        let code = raw_values.first().and_then(Value::as_i64).unwrap_or_default();

        // В событии 4 поля сообщения идут в фиксированном порядке VK Long Poll.
        let (event_type, object) = if code == 4 {
            let message = json!({
                "id": raw_values.get(1).and_then(Value::as_i64),
                "peer_id": raw_values.get(3).and_then(Value::as_i64),
                "date": raw_values.get(4).and_then(Value::as_i64),
                "text": raw_values.get(5).and_then(Value::as_str),
                "from_id": extra_value(raw_values, "from")
                    .or_else(|| extra_value(raw_values, "from_id"))
                    .and_then(value_as_i64),
                "conversation_message_id": extra_value(raw_values, "conversation_message_id")
                    .or_else(|| extra_value(raw_values, "cmid"))
                    .and_then(value_as_i64),
                "payload": extra_value(raw_values, "payload").and_then(parse_payload),
                "attachments": raw_values
                    .get(7)
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
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

fn extra_value(values: &[Value], key: &str) -> Option<Value> {
    let extra = values.get(6)?;

    match extra {
        Value::Object(object) => object.get(key).cloned(),
        Value::Array(items) => items.iter().find_map(|item| {
            let pair = item.as_str()?;
            let (name, value) = pair.split_once('=')?;
            (name == key).then(|| Value::String(value.to_owned()))
        }),
        Value::String(extra) => extra.split('&').find_map(|pair| {
            let (name, value) = pair.split_once('=')?;
            (name == key).then(|| Value::String(value.to_owned()))
        }),
        _ => None,
    }
}

fn value_as_i64(value: Value) -> Option<i64> {
    value
        .as_i64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
}

fn parse_payload(value: Value) -> Option<Value> {
    match value {
        Value::String(value) => serde_json::from_str(&value).ok().or(Some(Value::String(value))),
        value => Some(value),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Event;

    #[test]
    fn long_poll_message_parses_sender_and_attachments() {
        let event = Event::from_long_poll(
            123,
            json!([
                4,
                10,
                0,
                2000000001_i64,
                1720000000,
                "Привет",
                ["from=42", "conversation_message_id=7", "payload={\"action\":\"open\"}"],
                [{ "type": "photo" }]
            ]),
        );

        let message = event.message().unwrap();
        assert_eq!(message.id, Some(10));
        assert_eq!(message.peer_id, Some(2000000001));
        assert_eq!(message.from_id, Some(42));
        assert_eq!(message.conversation_message_id, Some(7));
        assert_eq!(message.payload.unwrap()["action"], "open");
        assert_eq!(message.attachments.len(), 1);
    }
}
