use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Сообщение ВКонтакте.
///
/// Поля, которые VK может не прислать в конкретном типе события, представлены
/// как Option. Неизвестные поля сохраняются в extra.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i64>,
    pub date: Option<i64>,
    pub peer_id: Option<i64>,
    pub from_id: Option<i64>,
    pub text: Option<String>,
    pub conversation_message_id: Option<i64>,
    pub payload: Option<Value>,
    #[serde(default)]
    pub attachments: Vec<Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
