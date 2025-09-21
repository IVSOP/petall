use uuid::Uuid;
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub image: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
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
