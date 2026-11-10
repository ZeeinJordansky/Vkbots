use async_trait::async_trait;

use crate::{fsm::Session, Result};

use super::SessionKey;

/// Интерфейс хранилища FSM.
///
/// Реализуйте этот trait для Redis, PostgreSQL или другого постоянного хранилища.
#[async_trait]
pub trait StateStorage: Send + Sync {
    async fn load(&self, key: &SessionKey) -> Result<Option<Session>>;
    async fn save(&self, key: &SessionKey, session: Session) -> Result<()>;
    async fn delete(&self, key: &SessionKey) -> Result<()>;
}
