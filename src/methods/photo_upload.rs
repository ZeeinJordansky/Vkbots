use std::{path::Path, time::Duration};

use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use crate::{api::ApiClient, Error, Result};

use super::{
    photos_get_messages_upload_server::PhotosGetMessagesUploadServerParams,
    photos_save_messages_photo::{PhotosSaveMessagesPhotoParams, SavedPhoto},
};

#[derive(Debug, Deserialize)]
struct PhotoUploadResult {
    server: i64,
    photo: String,
    hash: String,
}

impl ApiClient {
    /// Загружает локальный файл фотографии в диалог и сохраняет его в VK.
    ///
    /// Возвращает готовое вложение, которое можно передать в messages.send.
    pub async fn upload_message_photo(
        &self,
        peer_id: i64,
        path: impl AsRef<Path>,
    ) -> Result<SavedPhoto> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::InvalidParameter("не удалось определить имя фотографии".into()))?
            .to_owned();
        let bytes = tokio::fs::read(path).await?;

        let upload_server = self
            .photos_get_messages_upload_server(&PhotosGetMessagesUploadServerParams { peer_id })
            .await?;

        let mime = image_mime_type(path);
        let part = Part::bytes(bytes)
            .file_name(file_name)
            .mime_str(mime)?;
        let form = Form::new().part("photo", part);

        let response = self
            .http_client()
            .post(&upload_server.upload_url)
            .timeout(Duration::from_secs(120))
            .multipart(form)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(Error::HttpStatus {
                status: status.as_u16(),
                body,
            });
        }

        let uploaded: PhotoUploadResult = serde_json::from_str(&body)?;
        if uploaded.photo.is_empty() || uploaded.photo == "[]" {
            return Err(Error::Upload("сервер загрузки VK вернул пустое поле photo".into()));
        }

        let saved = self
            .photos_save_messages_photo(&PhotosSaveMessagesPhotoParams {
                server: uploaded.server,
                photo: uploaded.photo,
                hash: uploaded.hash,
                caption: None,
            })
            .await?;

        saved
            .into_iter()
            .next()
            .ok_or_else(|| Error::Upload("VK не вернул сохранённую фотографию".into()))
    }
}

fn image_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "image/jpeg",
    }
}
