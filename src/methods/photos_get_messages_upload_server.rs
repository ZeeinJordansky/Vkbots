use serde::{Deserialize, Serialize};

use crate::{api::ApiClient, Result};

/// Параметры photos.getMessagesUploadServer.
#[derive(Debug, Clone, Serialize)]
pub struct PhotosGetMessagesUploadServerParams {
    pub peer_id: i64,
}

/// Ответ с адресом сервера загрузки фотографии.
#[derive(Debug, Clone, Deserialize)]
pub struct PhotoUploadServer {
    pub upload_url: String,
    #[serde(default)]
    pub album_id: Option<i64>,
    #[serde(default)]
    pub user_id: Option<i64>,
}

impl ApiClient {
    /// Получает адрес сервера для загрузки фотографии в диалог.
    pub async fn photos_get_messages_upload_server(
        &self,
        params: &PhotosGetMessagesUploadServerParams,
    ) -> Result<PhotoUploadServer> {
        self.call("photos.getMessagesUploadServer", params).await
    }
}
