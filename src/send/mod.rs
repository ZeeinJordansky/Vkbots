mod answer;
mod builder;
mod common;
mod reply;

pub use builder::MessageBuilder;

use crate::api::ApiClient;

/// Отправка сообщений.
#[derive(Clone)]
pub struct Send {
    pub(crate) api: ApiClient,
}

impl Send {
    pub(crate) fn new(api: ApiClient) -> Self {
        Self { api }
    }

    /// Начинает отправку сообщения в указанный диалог.
    pub fn to(&self, peer_id: i64) -> MessageBuilder {
        MessageBuilder::new(self.api.clone(), peer_id)
    }
}
