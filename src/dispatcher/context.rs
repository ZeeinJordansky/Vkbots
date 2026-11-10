use serde::{de::DeserializeOwned, Serialize};

use crate::{fsm::{SessionKey, StateMachine}, models::{Event, Message}, Result, VkBot};

/// Контекст текущего обновления, передаваемый обработчику.
#[derive(Clone)]
pub struct Context {
    pub bot: VkBot,
    pub event: Event,
    pub message: Option<Message>,
    state: StateMachine,
}

impl Context {
    pub(crate) fn new(bot: VkBot, event: Event, state: StateMachine) -> Self {
        let message = event.message();
        Self {
            bot,
            event,
            message,
            state,
        }
    }

    /// Возвращает аргументы команды после её имени.
    pub fn command_args(&self) -> Vec<String> {
        self.message
            .as_ref()
            .and_then(|message| message.text.as_deref())
            .map(|text| {
                text.split_whitespace()
                    .skip(1)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Возвращает ключ FSM для текущего пользователя и диалога.
    pub fn session_key(&self) -> Option<SessionKey> {
        self.message.as_ref().and_then(SessionKey::from_message)
    }

    /// Устанавливает состояние текущей сессии.
    pub async fn set_state(&self, value: impl Into<String>) -> Result<()> {
        let key = self.session_key().ok_or_else(|| {
            crate::Error::InvalidParameter("у события нет данных пользователя для FSM".into())
        })?;
        self.state.set_state(&key, value).await
    }

    /// Возвращает состояние текущей сессии.
    pub async fn get_state(&self) -> Result<Option<String>> {
        let Some(key) = self.session_key() else {
            return Ok(None);
        };
        self.state.state(&key).await
    }

    /// Сохраняет произвольные данные текущей сессии.
    pub async fn set_data<T: Serialize>(&self, data: &T) -> Result<()> {
        let key = self.session_key().ok_or_else(|| {
            crate::Error::InvalidParameter("у события нет данных пользователя для FSM".into())
        })?;
        self.state.set_data(&key, data).await
    }

    /// Читает данные текущей сессии.
    pub async fn data<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        let Some(key) = self.session_key() else {
            return Ok(None);
        };
        self.state.data(&key).await
    }

    /// Одновременно меняет состояние и сохраняет данные.
    pub async fn transition<T: Serialize>(
        &self,
        state: impl Into<String>,
        data: &T,
    ) -> Result<()> {
        let key = self.session_key().ok_or_else(|| {
            crate::Error::InvalidParameter("у события нет данных пользователя для FSM".into())
        })?;
        self.state.transition(&key, state, data).await
    }

    /// Завершает сценарий и удаляет сессию.
    pub async fn clear_state(&self) -> Result<()> {
        let key = self.session_key().ok_or_else(|| {
            crate::Error::InvalidParameter("у события нет данных пользователя для FSM".into())
        })?;
        self.state.clear(&key).await
    }

    /// Отвечает на сообщение, которое вызвало обработчик.
    pub async fn reply(&self, text: impl Into<String>) -> Result<i64> {
        let message = self.message.as_ref().ok_or_else(|| {
            crate::Error::InvalidParameter("событие не содержит сообщения".into())
        })?;
        self.bot.send.reply(message, text).await
    }

    /// Отправляет сообщение в текущий диалог.
    pub async fn answer(&self, text: impl Into<String>) -> Result<i64> {
        let peer_id = self
            .message
            .as_ref()
            .and_then(|message| message.peer_id)
            .ok_or_else(|| crate::Error::InvalidParameter("у события отсутствует peer_id".into()))?;
        self.bot.send.answer(peer_id, text).await
    }
}
