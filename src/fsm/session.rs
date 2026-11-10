use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{models::Message, Result};

use super::{MemoryStorage, StateStorage};

/// Ключ сессии: пользователь внутри конкретного диалога.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionKey {
    pub peer_id: i64,
    pub user_id: i64,
}

impl SessionKey {
    pub fn new(peer_id: i64, user_id: i64) -> Self {
        Self { peer_id, user_id }
    }

    pub fn from_message(message: &Message) -> Option<Self> {
        Some(Self {
            peer_id: message.peer_id?,
            user_id: message.from_id?,
        })
    }
}

/// Данные пользовательской сессии.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub state: Option<String>,
    #[serde(default)]
    pub data: Value,
}

/// Менеджер конечного автомата состояний.
#[derive(Clone)]
pub struct StateMachine {
    storage: Arc<dyn StateStorage>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    /// Создаёт FSM с хранилищем в памяти.
    pub fn new() -> Self {
        Self::with_storage(Arc::new(MemoryStorage::new()))
    }

    /// Создаёт FSM с пользовательским хранилищем.
    pub fn with_storage(storage: Arc<dyn StateStorage>) -> Self {
        Self { storage }
    }

    /// Получает полную сессию.
    pub async fn get(&self, key: &SessionKey) -> Result<Option<Session>> {
        self.storage.load(key).await
    }

    /// Получает текущее состояние пользователя.
    pub async fn state(&self, key: &SessionKey) -> Result<Option<String>> {
        Ok(self.storage.load(key).await?.and_then(|session| session.state))
    }

    /// Устанавливает состояние, сохраняя данные сессии.
    pub async fn set_state(&self, key: &SessionKey, state: impl Into<String>) -> Result<()> {
        let mut session = self.storage.load(key).await?.unwrap_or_default();
        session.state = Some(state.into());
        self.storage.save(key, session).await
    }

    /// Сохраняет пользовательские данные, не меняя состояние.
    pub async fn set_data<T: Serialize>(&self, key: &SessionKey, data: &T) -> Result<()> {
        let mut session = self.storage.load(key).await?.unwrap_or_default();
        session.data = serde_json::to_value(data)?;
        self.storage.save(key, session).await
    }

    /// Меняет состояние и данные сессии одной операцией сохранения.
    pub async fn transition<T: Serialize>(
        &self,
        key: &SessionKey,
        state: impl Into<String>,
        data: &T,
    ) -> Result<()> {
        let session = Session {
            state: Some(state.into()),
            data: serde_json::to_value(data)?,
        };
        self.storage.save(key, session).await
    }

    /// Возвращает данные сессии в выбранный тип.
    pub async fn data<T: DeserializeOwned>(&self, key: &SessionKey) -> Result<Option<T>> {
        let Some(session) = self.storage.load(key).await? else {
            return Ok(None);
        };

        if session.data.is_null() {
            return Ok(None);
        }

        Ok(Some(serde_json::from_value(session.data)?))
    }

    /// Удаляет сессию и сбрасывает состояние пользователя.
    pub async fn clear(&self, key: &SessionKey) -> Result<()> {
        self.storage.delete(key).await
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionKey, StateMachine};

    #[tokio::test]
    async fn state_machine_saves_state_and_data() {
        let fsm = StateMachine::new();
        let key = SessionKey::new(2000000001, 42);

        fsm.transition(&key, "await_phone", &serde_json::json!({"name": "Роман"}))
            .await
            .unwrap();

        assert_eq!(fsm.state(&key).await.unwrap().as_deref(), Some("await_phone"));
        let data: serde_json::Value = fsm.data(&key).await.unwrap().unwrap();
        assert_eq!(data["name"], "Роман");
    }

    #[tokio::test]
    async fn clearing_session_removes_state_and_data() {
        let fsm = StateMachine::new();
        let key = SessionKey::new(2000000001, 42);
        fsm.set_state(&key, "await_name").await.unwrap();
        fsm.clear(&key).await.unwrap();

        assert!(fsm.get(&key).await.unwrap().is_none());
    }
}
