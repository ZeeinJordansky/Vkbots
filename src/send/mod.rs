mod answer;
mod builder;
mod common;
mod reply;

pub use answer::Answer;
pub use builder::MessageBuilder;
pub use reply::Reply;

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

    /// Отправляет сообщение без реплая.
    pub async fn answer(&self, peer_id: i64, text: impl Into<String>) -> crate::Result<i64> {
        self.to(peer_id).text(text).send().await
    }

    /// Отправляет ответ на полученное сообщение.
    pub async fn reply(
        &self,
        message: &crate::Message,
        text: impl Into<String>,
    ) -> crate::Result<i64> {
        let peer_id = message.peer_id.ok_or_else(|| {
            crate::Error::InvalidParameter(
                "у сообщения отсутствует peer_id, отправить реплай нельзя".to_owned(),
            )
        })?;
        let reply_to = message.id.ok_or_else(|| {
            crate::Error::InvalidParameter(
                "у сообщения отсутствует id, отправить реплай нельзя".to_owned(),
            )
        })?;

        self.to(peer_id).text(text).reply_to(reply_to).send().await
    }
}
