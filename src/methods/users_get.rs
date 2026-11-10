use serde::{Deserialize, Serialize};

use crate::{api::ApiClient, Result};

/// Параметры метода users.get.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UsersGetParams {
    pub user_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_case: Option<String>,
}

/// Основные поля пользователя ВКонтакте.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    #[serde(default)]
    pub is_closed: Option<bool>,
    #[serde(default)]
    pub can_access_closed: Option<bool>,
    #[serde(default)]
    pub deactivated: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ApiClient {
    /// Получает пользователей по их ID.
    pub async fn users_get(&self, params: &UsersGetParams) -> Result<Vec<User>> {
        self.call("users.get", params).await
    }
}
