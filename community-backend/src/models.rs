use std::ops::Range;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Duration, NaiveDateTime, Timelike};
use rand::Rng;
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

pub fn rng_big_decimal_range(range: &Range<BigDecimal>) -> BigDecimal {
    let mut rng = rand::thread_rng();
    let diff = &range.end - &range.start;
    let factor: f64 = rng.gen_range(0.0..1.0);
    let factor_bd = BigDecimal::from_f64(factor).expect("NaN return in factor");
    &range.start + factor_bd * diff
}

impl EnergyRecord {
    pub fn random(user_id: Uuid, community_id: Uuid, start: NaiveDateTime) -> Self {
        let energy_range = BigDecimal::from(50)..BigDecimal::from(100);

        let price_min = BigDecimal::from(1) / BigDecimal::from(10000);
        let price_max = BigDecimal::from(2) / BigDecimal::from(10000);
        let price_range = price_min..price_max;

        let generated = rng_big_decimal_range(&energy_range);
        let consumed = rng_big_decimal_range(&energy_range);
        let consumer_price = rng_big_decimal_range(&price_range);
        let seller_price = rng_big_decimal_range(&price_range);

        EnergyRecord {
            id: Uuid::new_v4(),
            user_id,
            community_id,
            generated,
            consumed,
            consumer_price,
            seller_price,
            start,
        }
    }

    pub fn random_vec(
        user_id: Uuid,
        community_id: Uuid,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Vec<EnergyRecord> {
        let mut records = Vec::new();

        // Round start time to the next 15-minute interval (:00, :15, :30, :45)
        let mut time = start
            .with_minute(0)
            .expect("0 <= minutes < 60")
            .with_second(0)
            .expect("0 <= seconds < 60")
            .with_nanosecond(0)
            .expect("nanoseconds < 200.000.000");

        while time < end {
            let record = Self::random(user_id, community_id, time);

            records.push(record);
            time += Duration::minutes(15);
        }

        records
    }
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
