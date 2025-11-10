use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
pub struct UserCommunity {
    pub user_id: Uuid,
    pub community_id: Uuid,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "auth_provider", rename_all = "lowercase")]
pub enum AuthProvider {
    Email,
    Google,
    GitHub,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub id: String,
    pub provider: AuthProvider,
    pub user_id: Uuid,
    pub hashed_password: Option<String>,
}
