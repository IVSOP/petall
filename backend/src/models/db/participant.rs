use uuid::Uuid;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "lowercase")]
pub enum ParticipantRole {
    User,
    Manager,
    UserManager,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct Participant {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub supplier: Uuid,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantAlias {
    pub participant: Uuid,
    pub alias: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantCommunity {
    pub participant: Uuid,
    pub community: Uuid,
    pub role: ParticipantRole,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Energy {
    pub id: Uuid,
    pub participant: Uuid,
    pub community: Uuid,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub generated: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumed: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub coeficient: BigDecimal,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}