use std::{collections::BTreeMap, fmt};

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use crate::error::{Error, Result, VkApiError};

const API_URL: &str = "https://api.vk.com/method";

/// Клиент запросов к VK API.
#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    token: String,
    version: String,
}

impl ApiClient {
    /// Создаёт API-клиент с версией VK API по умолчанию.
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_client(
            token,
            crate::api::DEFAULT_API_VERSION.to_owned(),
            Client::new(),
        )
    }

    pub(crate) fn with_client(
        token: impl Into<String>,
        version: impl Into<String>,
        http: Client,
    ) -> Self {
        Self {
            http,
            token: token.into(),
            version: version.into(),
        }
    }

    /// Возвращает установленную версию VK API.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Выполняет любой доступный текущему токену метод VK API.
    ///
    /// Параметры передаются сериализуемой структурой или JSON-объектом.
    /// Поля access_token и v библиотека добавляет самостоятельно.
    pub async fn call<T, P>(&self, method: &str, params: &P) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        if !method.split('.').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '_')
        }) {
            return Err(Error::InvalidParameter(format!(
                "недопустимое имя метода VK API: {method}"
            )));
        }

        let value = serde_json::to_value(params)?;
        let mut values = match value {
            Value::Object(values) => values,
            _ => {
                return Err(Error::InvalidParameter(
                    "параметры VK API должны быть JSON-объектом".to_owned(),
                ));
            }
        };

        values.insert("access_token".to_owned(), Value::String(self.token.clone()));
        values.insert("v".to_owned(), Value::String(self.version.clone()));

        let form = into_form(values)?;
        let response = self
            .http
            .post(format!("{API_URL}/{method}"))
            .form(&form)
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

        let envelope: ApiEnvelope<T> = serde_json::from_str(&body)?;
        if let Some(error) = envelope.error {
            return Err(Error::VkApi(error));
        }

        envelope
            .response
            .ok_or_else(|| Error::Protocol(format!("метод {method} не вернул поле response")))
    }

    pub(crate) fn http_client(&self) -> &Client {
        &self.http
    }
}

impl fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiClient")
            .field("token", &"[скрыт]")
            .field("version", &self.version)
            .finish()
    }
}

#[derive(serde::Deserialize)]
struct ApiEnvelope<T> {
    response: Option<T>,
    error: Option<VkApiError>,
}

fn into_form(values: Map<String, Value>) -> Result<Vec<(String, String)>> {
    let values: BTreeMap<_, _> = values.into_iter().collect();
    let mut form = Vec::with_capacity(values.len());

    for (key, value) in values {
        let value = match value {
            Value::Null => continue,
            Value::String(value) => value,
            Value::Number(value) => value.to_string(),
            Value::Bool(value) => if value { "1" } else { "0" }.to_owned(),
            other => serde_json::to_string(&other)?,
        };
        form.push((key, value));
    }

    Ok(form)
}

#[cfg(test)]
mod tests {
    use super::into_form;
    use serde_json::json;

    #[test]
    fn form_values_are_encoded_as_vk_expects() {
        let value = json!({
            "peer_id": 123,
            "message": "Привет",
            "disable_mentions": true,
            "attachment": null
        });
        let form = into_form(value.as_object().unwrap().clone()).unwrap();
        assert_eq!(form.len(), 3);
        assert!(form.iter().any(|(key, value)| key == "peer_id" && value == "123"));
        assert!(form.iter().any(|(key, value)| key == "disable_mentions" && value == "1"));
        assert!(form.iter().any(|(key, value)| key == "message" && value == "Привет"));
    }
}
