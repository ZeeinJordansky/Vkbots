use serde::{Deserialize, Serialize};

use crate::{api::ApiClient, Result};

/// Параметры photos.saveMessagesPhoto.
#[derive(Debug, Clone, Serialize)]
pub struct PhotosSaveMessagesPhotoParams {
    pub server: i64,
    pub photo: String,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

/// Сохранённая фотография.
#[derive(Debug, Clone, Deserialize)]
pub struct SavedPhoto {
    pub id: i64,
    pub owner_id: i64,
    #[serde(default)]
    pub access_key: Option<String>,
    #[serde(default)]
    pub album_id: Option<i64>,
    #[serde(default)]
    pub date: Option<i64>,
}

impl SavedPhoto {
    /// Возвращает строку вложения, которую принимает messages.send.
    pub fn attachment(&self) -> String {
        match &self.access_key {
            Some(key) => format!("photo{}_{}_{}", self.owner_id, self.id, key),
            None => format!("photo{}_{}", self.owner_id, self.id),
        }
    }
}

impl ApiClient {
    /// Сохраняет загруженную фотографию в сообщениях.
    pub async fn photos_save_messages_photo(
        &self,
        params: &PhotosSaveMessagesPhotoParams,
    ) -> Result<Vec<SavedPhoto>> {
        self.call("photos.saveMessagesPhoto", params).await
    }
}
