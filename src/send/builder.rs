use std::path::Path;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::{api::ApiClient, Error, Result};

/// Настраиваемый запрос отправки сообщения.
pub struct MessageBuilder {
    api: ApiClient,
    params: Map<String, Value>,
}

impl MessageBuilder {
    pub(crate) fn new(api: ApiClient, peer_id: i64) -> Self {
        let mut params = Map::new();
        params.insert("peer_id".to_owned(), Value::from(peer_id));
        params.insert("random_id".to_owned(), Value::from(rand::random_range(1..=i32::MAX)));
        Self { api, params }
    }

    /// Устанавливает текст сообщения.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.insert("message".to_owned(), Value::String(text.into()));
        self
    }

    /// Устанавливает ID сообщения, на которое нужно ответить.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.insert("reply_to".to_owned(), Value::from(message_id));
        self
    }

    /// Устанавливает вложение в формате VK, например photo-1_123.
    pub fn attachment(mut self, attachment: impl Into<String>) -> Self {
        self.params.insert("attachment".to_owned(), Value::String(attachment.into()));
        self
    }

    /// Устанавливает список вложений VK.
    pub fn attachments<I, S>(mut self, attachments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let attachments = attachments.into_iter().map(Into::into).collect::<Vec<_>>();
        self.params.insert("attachment".to_owned(), Value::String(attachments.join(",")));
        self
    }

    /// Загружает локальное фото в VK и добавляет его к сообщению.
    pub async fn attach_local_photo(mut self, path: impl AsRef<Path>) -> Result<Self> {
        let peer_id = self
            .params
            .get("peer_id")
            .and_then(Value::as_i64)
            .ok_or_else(|| Error::InvalidParameter("не указан peer_id".into()))?;
        let photo = self.api.upload_message_photo(peer_id, path).await?;
        let attachment = photo.attachment();

        let current = self
            .params
            .get("attachment")
            .and_then(Value::as_str)
            .map(str::to_owned);

        let value = match current {
            Some(current) if !current.is_empty() => format!("{current},{attachment}"),
            _ => attachment,
        };
        self.params.insert("attachment".to_owned(), Value::String(value));
        Ok(self)
    }

    /// Устанавливает клавиатуру в формате VK.
    pub fn keyboard<T: Serialize>(mut self, keyboard: &T) -> Result<Self> {
        self.params.insert("keyboard".to_owned(), serde_json::to_value(keyboard)?);
        Ok(self)
    }

    /// Устанавливает payload сообщения или кнопки.
    pub fn payload<T: Serialize>(mut self, payload: &T) -> Result<Self> {
        self.params.insert("payload".to_owned(), serde_json::to_value(payload)?);
        Ok(self)
    }

    /// Устанавливает собственный идентификатор запроса.
    pub fn random_id(mut self, random_id: i32) -> Self {
        self.params.insert("random_id".to_owned(), Value::from(random_id));
        self
    }

    /// Отключает автоматическое распознавание ссылок в тексте.
    pub fn dont_parse_links(mut self, value: bool) -> Self {
        self.params.insert("dont_parse_links".to_owned(), Value::from(value));
        self
    }

    /// Отключает упоминания пользователей при отправке текста.
    pub fn disable_mentions(mut self, value: bool) -> Self {
        self.params.insert("disable_mentions".to_owned(), Value::from(value));
        self
    }

    /// Добавляет параметр messages.send, для которого ещё нет отдельного метода.
    pub fn parameter<T: Serialize>(mut self, name: impl Into<String>, value: &T) -> Result<Self> {
        let name = name.into();
        if name == "access_token" || name == "v" {
            return Err(Error::InvalidParameter(format!(
                "параметр {name} устанавливается библиотекой автоматически"
            )));
        }
        self.params.insert(name, serde_json::to_value(value)?);
        Ok(self)
    }

    /// Отправляет сообщение и возвращает его идентификатор.
    pub async fn send(self) -> Result<i64> {
        if !self.params.contains_key("message")
            && !self.params.contains_key("attachment")
            && !self.params.contains_key("sticker_id")
            && !self.params.contains_key("template")
        {
            return Err(Error::InvalidParameter(
                "нужно указать текст, вложение, стикер или шаблон".to_owned(),
            ));
        }

        self.api.call("messages.send", &Value::Object(self.params)).await
    }
}
