use crate::error::AppResult;
use crate::models::{AuthProvider, EnergyRecord, User, UserCommunity};
use crate::{AppState, auth};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use uuid::Uuid;

impl AppState {
    pub async fn insert_random_energy_records(&self) -> sqlx::Result<()> {
        // clamp current time to nearest 15-minute mark
        let now = Utc::now();
        let minutes = now.minute() - (now.minute() % 15);
        let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .expect("valid date");
        let time = NaiveTime::from_hms_opt(now.hour(), minutes, 0)
            .expect("valid time");
        let start_rounded: NaiveDateTime = date.and_time(time);

        // get user/community pairs
        let pairs = sqlx::query!(
            r#"
            SELECT user_id, community_id
            FROM community_user
            "#
        )
        .fetch_all(&self.pg_pool)
        .await?;

        let mut random_records = Vec::with_capacity(pairs.len());
        for pair in pairs.iter() {
            random_records.push(EnergyRecord::random(pair.user_id, pair.community_id, start_rounded));
        }

        self.insert_energy_records(&random_records).await
    }
}
