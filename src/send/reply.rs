use super::Send;

impl Send {
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
