use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EnergyRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Uuid,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub generated: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumed: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumer_price: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub seller_price: BigDecimal,
    pub start: NaiveDateTime,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Participant,
    Manager,
    UserManager,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    //pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserCommunity {
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub id: String, // "email:user@example.com" or "oauth:github:123456"
    pub user_id: Uuid,
    pub hashed_password: Option<String>, // Null for OAuth users
}

impl Key {
    pub fn email_key_id(email: &str) -> String {
        format!("email:{}", email)
    }

    pub fn oauth_key_id(provider: &str, provider_user_id: &str) -> String {
        format!("oauth:{}:{}", provider, provider_user_id)
    }
}
