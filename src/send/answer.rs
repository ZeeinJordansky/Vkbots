use super::Send;

impl Send {
    /// Отправляет сообщение без реплая.
    pub async fn answer(&self, peer_id: i64, text: impl Into<String>) -> crate::Result<i64> {
        self.to(peer_id).text(text).send().await
    }
}
