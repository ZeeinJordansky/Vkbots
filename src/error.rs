use thiserror::Error;

/// Результат операции библиотеки.
pub type Result<T> = std::result::Result<T, Error>;

/// Ошибка, возвращаемая VK API.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct VkApiError {
    pub error_code: i64,
    pub error_msg: String,
    #[serde(default)]
    pub request_params: Vec<VkApiRequestParam>,
}

/// Параметр запроса, указанный VK в описании ошибки.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct VkApiRequestParam {
    pub key: String,
    pub value: String,
}

/// Общий тип ошибок Vkbots.
#[derive(Debug, Error)]
pub enum Error {
    #[error("ошибка HTTP-запроса: {0}")]
    Http(#[from] reqwest::Error),
    #[error("ошибка разбора JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("ошибка VK API: {0}")]
    VkApi(VkApiError),
    #[error("VK вернул HTTP {status}: {body}")]
    HttpStatus { status: u16, body: String },
    #[error("неверный параметр: {0}")]
    InvalidParameter(String),
    #[error("неожиданный ответ VK: {0}")]
    Protocol(String),
    #[error("ошибка сетевого сервера: {0}")]
    Io(#[from] std::io::Error),
}

impl std::fmt::Display for VkApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VK API {}: {}", self.error_code, self.error_msg)
    }
}

impl std::error::Error for VkApiError {}
