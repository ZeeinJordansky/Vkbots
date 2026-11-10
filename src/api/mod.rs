mod limiter;
mod request;

pub use request::ApiClient;

/// Адрес VK API.
pub const API_URL: &str = "https://api.vk.com/method";

/// Версия VK API, используемая по умолчанию.
pub const DEFAULT_API_VERSION: &str = "5.199";
