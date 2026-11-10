use std::time::Duration;

use reqwest::Client;

use crate::{api::ApiClient, error::Result, send::Send};

/// Основной объект библиотеки. Его можно безопасно клонировать и передавать
/// между асинхронными задачами.
#[derive(Clone)]
pub struct VkBot {
    pub api: ApiClient,
    pub send: Send,
}

impl VkBot {
    /// Создаёт бота с настройками по умолчанию.
    pub fn new(token: impl Into<String>) -> Self {
        Self::from_api(ApiClient::new(token))
    }

    /// Создаёт объект настройки клиента.
    pub fn builder(token: impl Into<String>) -> VkBotBuilder {
        VkBotBuilder {
            token: token.into(),
            api_version: crate::api::DEFAULT_API_VERSION.to_owned(),
            timeout: Duration::from_secs(30),
        }
    }

    fn from_api(api: ApiClient) -> Self {
        let send = Send::new(api.clone());
        Self { api, send }
    }
}

/// Настройка основного клиента.
pub struct VkBotBuilder {
    token: String,
    api_version: String,
    timeout: Duration,
}

impl VkBotBuilder {
    /// Устанавливает версию VK API.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Устанавливает тайм-аут HTTP-запросов.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Создаёт клиент.
    pub fn build(self) -> Result<VkBot> {
        let http = Client::builder().timeout(self.timeout).build()?;
        let api = ApiClient::with_client(self.token, self.api_version, http);
        Ok(VkBot::from_api(api))
    }
}

impl std::fmt::Debug for VkBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VkBot").field("api", &self.api).finish()
    }
}
