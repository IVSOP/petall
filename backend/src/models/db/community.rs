use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

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
    pub consumer_price: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub seller_price: BigDecimal,
    pub start: NaiveDateTime,
}
