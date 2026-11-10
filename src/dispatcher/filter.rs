use std::sync::Arc;

use serde_json::Value;

use crate::Message;

/// Условие отбора входящих сообщений.
#[derive(Clone)]
pub struct Filter {
    predicate: Arc<dyn Fn(&Message) -> bool + Send + Sync>,
}

impl Filter {
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(&Message) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Arc::new(predicate),
        }
    }

    pub fn check(&self, message: &Message) -> bool {
        (self.predicate)(message)
    }
}

/// Точное совпадение текста без учёта регистра и внешних пробелов.
pub fn text(expected: impl Into<String>) -> Filter {
    let expected = expected.into().trim().to_lowercase();
    Filter::new(move |message| {
        message
            .text
            .as_deref()
            .map(|text| text.trim().to_lowercase() == expected)
            .unwrap_or(false)
    })
}

/// Проверяет, содержит ли текст заданную строку.
pub fn contains(expected: impl Into<String>) -> Filter {
    let expected = expected.into().to_lowercase();
    Filter::new(move |message| {
        message
            .text
            .as_deref()
            .map(|text| text.to_lowercase().contains(&expected))
            .unwrap_or(false)
    })
}

/// Проверяет начало текста.
pub fn starts_with(expected: impl Into<String>) -> Filter {
    let expected = expected.into().to_lowercase();
    Filter::new(move |message| {
        message
            .text
            .as_deref()
            .map(|text| text.trim_start().to_lowercase().starts_with(&expected))
            .unwrap_or(false)
    })
}

/// Пропускает только сообщения с непустым текстом.
pub fn any_text() -> Filter {
    Filter::new(|message| {
        message.text.as_deref().is_some_and(|text| !text.trim().is_empty())
    })
}

/// Сравнивает payload сообщения с ожидаемым JSON-значением.
pub fn payload(expected: Value) -> Filter {
    Filter::new(move |message| message.payload.as_ref() == Some(&expected))
}
