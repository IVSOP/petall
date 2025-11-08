use std::time::Duration;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use tokio::time::Instant;
use tracing::{error, info};

use crate::{AppState, models::EnergyRecord};

/// Starts a scheduler that runs every quarter-hour (00, 15, 30, 45)
pub async fn run_periodic_seed_task(state: AppState) {
    // --- Compute the first tick aligned to next quarter-hour ---
    let now = Utc::now();
    let minutes = now.minute();
    let seconds = now.second();

    // Compute how many minutes until the next multiple of 15
    // This means this is only accurate to the minute. However, when records are inserted, seconds are always zeroed out
    let next_quarter = ((minutes / 15) + 1) * 15 % 60;
    let minutes_until_next = if next_quarter > minutes {
        next_quarter - minutes
    } else {
        60 - minutes
    };

    let initial_delay_secs = minutes_until_next * 60 - seconds;
    let initial_delay = Duration::from_secs(initial_delay_secs as u64);
    let start_instant = Instant::now() + initial_delay;

    // Create a 15-minute interval aligned to that start time
    let mut interval = tokio::time::interval_at(start_instant, Duration::from_secs(15 * 60));

    info!("Scheduler will start in {initial_delay:?} (at next quarter-hour boundary)");

    loop {
        interval.tick().await;
        info!("Seeding records");

        if let Err(e) = insert_random_energy_records(&state).await {
            error!("Error inserting random energy records: {}", e);
        }
    }
}

async fn insert_random_energy_records(state: &AppState) -> sqlx::Result<()> {
    // clamp current time to nearest 15-minute mark
    let now = Utc::now();
    let minutes = now.minute() - (now.minute() % 15);
    let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).expect("valid date");
    let time = NaiveTime::from_hms_opt(now.hour(), minutes, 0).expect("valid time");
    let start_rounded: NaiveDateTime = date.and_time(time);

    // get user/community pairs
    let pairs = sqlx::query!(
        r#"
        SELECT user_id, community_id
        FROM community_user
        "#
    )
    .fetch_all(&state.pg_pool)
    .await?;

    let mut random_records = Vec::with_capacity(pairs.len());
    for pair in pairs.iter() {
        random_records.push(EnergyRecord::random(
            pair.user_id,
            pair.community_id,
            start_rounded,
        ));
    }

    state.insert_energy_records(&random_records).await
}
