use std::{collections::BTreeMap, fmt, time::Duration};

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use tokio::time::sleep;

use crate::error::{Error, Result, VkApiError};

use super::limiter::RateLimiter;

/// Клиент запросов к VK API.
#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    token: String,
    version: String,
    base_url: String,
    limiter: RateLimiter,
    max_retries: u32,
    retry_base_delay: Duration,
    retry_max_delay: Duration,
}

impl ApiClient {
    /// Создаёт API-клиент с версией VK API и ограничением по умолчанию.
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_policy(
            token,
            crate::api::DEFAULT_API_VERSION.to_owned(),
            crate::api::API_URL.to_owned(),
            Client::new(),
            3,
            4,
            Duration::from_millis(300),
            Duration::from_secs(8),
        )
    }

    pub(crate) fn with_policy(
        token: impl Into<String>,
        version: impl Into<String>,
        base_url: impl Into<String>,
        http: Client,
        requests_per_second: u32,
        max_retries: u32,
        retry_base_delay: Duration,
        retry_max_delay: Duration,
    ) -> Self {
        Self {
            http,
            token: token.into(),
            version: version.into(),
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            limiter: RateLimiter::new(requests_per_second),
            max_retries,
            retry_base_delay,
            retry_max_delay,
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
    ///
    /// Ошибки VK с кодами 6, 9 и 29 повторяются с экспоненциальной задержкой.
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

        for attempt in 0..=self.max_retries {
            self.limiter.wait_turn().await;

            let response = self
                .http
                .post(format!("{}/{method}", self.base_url))
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
                if is_temporary_vk_error(&error) && attempt < self.max_retries {
                    sleep(self.retry_delay(attempt)).await;
                    continue;
                }
                return Err(Error::VkApi(error));
            }

            return envelope
                .response
                .ok_or_else(|| Error::Protocol(format!("метод {method} не вернул поле response")));
        }

        Err(Error::Protocol("повторные запросы завершились без ответа".to_owned()))
    }

    fn retry_delay(&self, attempt: u32) -> Duration {
        let multiplier = 1u32.checked_shl(attempt.min(16)).unwrap_or(u32::MAX);
        self.retry_base_delay
            .saturating_mul(multiplier)
            .min(self.retry_max_delay)
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
            .field("base_url", &self.base_url)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

#[derive(serde::Deserialize)]
struct ApiEnvelope<T> {
    response: Option<T>,
    error: Option<VkApiError>,
}

fn is_temporary_vk_error(error: &VkApiError) -> bool {
    matches!(error.error_code, 6 | 9 | 29)
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
            Value::Array(values)
                if values.iter().all(|value| value.is_string() || value.is_number() || value.is_boolean()) =>
            {
                values
                    .into_iter()
                    .map(|value| match value {
                        Value::String(value) => value,
                        Value::Number(value) => value.to_string(),
                        Value::Bool(value) => if value { "1" } else { "0" }.to_owned(),
                        _ => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            }
            other => serde_json::to_string(&other)?,
        };
        form.push((key, value));
    }

    Ok(form)
}

#[cfg(test)]
mod tests {
    use super::{into_form, is_temporary_vk_error};
    use crate::error::VkApiError;
    use serde_json::json;

    #[test]
    fn form_values_are_encoded_as_vk_expects() {
        let value = json!({
            "peer_id": 123,
            "message": "Привет",
            "disable_mentions": true,
            "attachment": null,
            "user_ids": [1, 2, 3]
        });
        let form = into_form(value.as_object().unwrap().clone()).unwrap();
        assert_eq!(form.len(), 4);
        assert!(form.iter().any(|(key, value)| key == "peer_id" && value == "123"));
        assert!(form.iter().any(|(key, value)| key == "disable_mentions" && value == "1"));
        assert!(form.iter().any(|(key, value)| key == "message" && value == "Привет"));
        assert!(form.iter().any(|(key, value)| key == "user_ids" && value == "1,2,3"));
    }

    #[test]
    fn temporary_vk_errors_are_recognized() {
        for code in [6, 9, 29] {
            let error = VkApiError {
                error_code: code,
                error_msg: "temporary".into(),
                request_params: Vec::new(),
            };
            assert!(is_temporary_vk_error(&error));
        }

        let permanent = VkApiError {
            error_code: 5,
            error_msg: "bad token".into(),
            request_params: Vec::new(),
        };
        assert!(!is_temporary_vk_error(&permanent));
    }
}
