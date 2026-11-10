use std::time::Duration;

use reqwest::Client;

use crate::{api::ApiClient, dispatcher::Dispatcher, error::Result, send::Send};

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
        Self::builder(token).build().expect("стандартный HTTP-клиент должен создаваться без ошибок")
    }

    /// Создаёт объект настройки клиента.
    pub fn builder(token: impl Into<String>) -> VkBotBuilder {
        VkBotBuilder {
            token: token.into(),
            api_version: crate::api::DEFAULT_API_VERSION.to_owned(),
            api_base_url: crate::api::API_URL.to_owned(),
            timeout: Duration::from_secs(30),
            requests_per_second: 3,
            max_retries: 4,
            retry_base_delay: Duration::from_millis(300),
            retry_max_delay: Duration::from_secs(8),
        }
    }

    /// Создаёт диспетчер обработчиков для этого бота.
    pub fn dispatcher(&self) -> Dispatcher {
        Dispatcher::new(self.clone())
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
    api_base_url: String,
    timeout: Duration,
    requests_per_second: u32,
    max_retries: u32,
    retry_base_delay: Duration,
    retry_max_delay: Duration,
}

impl VkBotBuilder {
    /// Устанавливает версию VK API.
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }

    /// Устанавливает базовый URL VK API или совместимого прокси.
    ///
    /// Обычно менять его не требуется. Параметр полезен для тестовых серверов.
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = url.into().trim_end_matches('/').to_owned();
        self
    }

    /// Устанавливает тайм-аут HTTP-запросов.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Устанавливает общий предел запросов в секунду для этого клиента.
    ///
    /// Ограничитель общий для всех клонов клиента. Значение по умолчанию — 3.
    pub fn requests_per_second(mut self, value: u32) -> Self {
        self.requests_per_second = value.max(1);
        self
    }

    /// Устанавливает число повторов после временных ошибок VK API.
    pub fn max_retries(mut self, value: u32) -> Self {
        self.max_retries = value;
        self
    }

    /// Устанавливает начальную и максимальную задержку повторных запросов.
    pub fn retry_delays(mut self, base: Duration, maximum: Duration) -> Self {
        self.retry_base_delay = base;
        self.retry_max_delay = maximum.max(base);
        self
    }

    /// Создаёт клиент.
    pub fn build(self) -> Result<VkBot> {
        let http = Client::builder()
            .timeout(self.timeout)
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(16)
            .build()?;
        let api = ApiClient::with_policy(
            self.token,
            self.api_version,
            self.api_base_url,
            http,
            self.requests_per_second,
            self.max_retries,
            self.retry_base_delay,
            self.retry_max_delay,
        );
        Ok(VkBot::from_api(api))
    }
}

impl std::fmt::Debug for VkBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VkBot").field("api", &self.api).finish()
    }
}
