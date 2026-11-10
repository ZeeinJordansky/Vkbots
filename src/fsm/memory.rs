use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{fsm::{Session, SessionKey, StateStorage}, Result};

/// Хранилище состояний в памяти процесса.
///
/// Подходит для одного экземпляра бота и разработки. Данные пропадут после
/// перезапуска процесса. Для постоянного хранения реализуйте StateStorage.
#[derive(Clone, Default)]
pub struct MemoryStorage {
    sessions: Arc<RwLock<HashMap<SessionKey, Session>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn len(&self) -> usize {
        self.sessions.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.sessions.read().await.is_empty()
    }
}

#[async_trait]
impl StateStorage for MemoryStorage {
    async fn load(&self, key: &SessionKey) -> Result<Option<Session>> {
        Ok(self.sessions.read().await.get(key).cloned())
    }

    async fn save(&self, key: &SessionKey, session: Session) -> Result<()> {
        self.sessions.write().await.insert(key.clone(), session);
        Ok(())
    }

    async fn delete(&self, key: &SessionKey) -> Result<()> {
        self.sessions.write().await.remove(key);
        Ok(())
    }
}
