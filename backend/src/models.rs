use std::ops::Range;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Duration, NaiveDateTime};
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

// FIX: meter isto noutro sitio
pub fn rand_big_decimal_range(range: &Range<BigDecimal>) -> BigDecimal {
    let start = &range.start;
    let end = &range.end;

    let t = fastrand::f64();
    let t_big =
        BigDecimal::from_f64(t).expect("Could not convert random normalized f64 to big int");
    let interval_size = end - start.clone();

    start + (t_big * interval_size)
}

impl EnergyRecord {
    // TODO: use RangeBounds instead
    pub fn random_vec(
        user_id: &Uuid,
        community_id: &Uuid,
        date_range: Range<NaiveDateTime>,
        energy_range: Range<BigDecimal>,
        price_range: Range<BigDecimal>,
    ) -> Vec<EnergyRecord> {
        let mut records = Vec::new();

        let user_id = *user_id;
        let community_id = *community_id;

        let mut time = date_range.start;
        while time < date_range.end {
            let generated = rand_big_decimal_range(&energy_range);
            let consumed = rand_big_decimal_range(&energy_range);
            let consumer_price = rand_big_decimal_range(&price_range);
            let seller_price = rand_big_decimal_range(&price_range);

            let record = EnergyRecord {
                id: Uuid::new_v4(),
                user_id,
                community_id,
                generated,
                consumed,
                consumer_price,
                seller_price,
                start: time,
            };

            records.push(record);
            time += Duration::minutes(15);
        }

        records
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, sqlx::Type, PartialEq, Eq)]
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
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserCommunity {
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub role: UserRole,
}
