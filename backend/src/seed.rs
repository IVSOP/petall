use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use fake::{Fake, faker::internet::en::FreeEmail};
use names::Generator;
use rand::{Rng, seq::IteratorRandom};
use sqlx::postgres::PgPool;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, clap::Parser)]
pub struct SeedSettings {
    #[arg(long, default_value_t = 10)]
    communities: usize,
    #[arg(long, default_value_t = 5)]
    managers: usize,
    #[arg(long, default_value_t = 5)]
    users: usize,
    #[arg(long, default_value_t = 50)]
    energy_transfers: usize,
    #[arg(long, default_value_t = 6)]
    communities_per_user: usize,
    #[arg(long, default_value_t = 6)]
    communities_per_manager: usize,
}

pub async fn run_seed(pg_pool: &PgPool, seed_settings: SeedSettings) -> anyhow::Result<()> {
    let community_ids = seed_community(pg_pool, seed_settings.communities).await?;

    let manager_ids = seed_manager(pg_pool, seed_settings.managers).await?;

    let users_ids = seed_user(pg_pool, seed_settings.users).await?;

    seed_energytransfer(
        pg_pool,
        &community_ids,
        &users_ids,
        seed_settings.energy_transfers,
    )
    .await?;

    seed_user_community(
        pg_pool,
        &users_ids,
        &community_ids,
        seed_settings.communities_per_user,
    )
    .await?;

    seed_manager_community(
        pg_pool,
        &manager_ids,
        &community_ids,
        seed_settings.communities_per_manager,
    )
    .await?;

    Ok(())
}

async fn seed_community(pool: &PgPool, count: usize) -> anyhow::Result<Vec<Uuid>> {
    let mut generator = Generator::default();
    let mut community_ids = Vec::new();

    for _ in 0..count {
        let id = Uuid::new_v4();
        let entity = generator.next().unwrap();

        let result = sqlx::query!(
            r#"
            INSERT INTO "community" ("id", "entity")
            VALUES ($1, $2)
            ON CONFLICT ("entity") DO NOTHING
            "#,
            id,
            entity
        )
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            community_ids.push(id);
        }
    }

    Ok(community_ids)
}

async fn seed_user(pool: &PgPool, count: usize) -> anyhow::Result<Vec<Uuid>> {
    let mut generator = Generator::default();
    let mut user_ids = Vec::new();

    for _ in 0..count {
        let id: Uuid = Uuid::new_v4();
        let name = generator.next().unwrap();
        let email = FreeEmail().fake::<String>();

        let result = sqlx::query!(
            r#"
            INSERT INTO "user" ("id", "name", "email")
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
            id,
            name,
            email,
        )
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            user_ids.push(id);
        }
    }

    Ok(user_ids)
}

async fn seed_manager(pool: &PgPool, count: usize) -> anyhow::Result<Vec<Uuid>> {
    let mut generator = Generator::default();
    let mut manager_ids = Vec::new();

    for _ in 0..count {
        let id: Uuid = Uuid::new_v4();
        let name = generator.next().unwrap();
        let email = FreeEmail().fake::<String>();

        let result = sqlx::query!(
            r#"
            INSERT INTO "manager" ("id", "name", "email")
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
            id,
            name,
            email,
        )
        .execute(pool)
        .await?;

        if result.rows_affected() > 0 {
            manager_ids.push(id);
        }
    }

    Ok(manager_ids)
}

async fn seed_energytransfer(
    pool: &PgPool,
    communities: &Vec<Uuid>,
    users: &Vec<Uuid>,
    count: usize,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();

    for _ in 0..count {
        let id: Uuid = Uuid::new_v4();
        let community = communities.into_iter().choose(&mut rng).unwrap();
        let energy_wh = BigDecimal::from_str(&rng.random_range(10.0..5000.0).to_string())?;

        let start = (Utc::now() - Duration::days(rng.random_range(0..30))).naive_utc()
            - Duration::minutes(rng.random_range(10..120));
        let end = start + Duration::minutes(rng.random_range(10..120));

        let user_from = users.into_iter().choose(&mut rng).unwrap();
        let mut user_to = users.into_iter().choose(&mut rng).unwrap();

        while user_to == user_from {
            user_to = users.into_iter().choose(&mut rng).unwrap();
        }

        sqlx::query!(
            r#"
            INSERT INTO "energytransfer" ("id", "user_from", "user_to", "community", "energy_wh", "start", "end")
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT DO NOTHING
            "#,
            id,
            user_from,
            user_to,
            community,
            energy_wh,
            start,
            end
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn seed_user_community(
    pool: &PgPool,
    users: &Vec<Uuid>,
    communities: &Vec<Uuid>,
    count: usize,
) -> anyhow::Result<Vec<(Uuid, Uuid)>> {
    let mut rng = rand::rng();
    let mut inserted_pairs = Vec::new();

    for &user_id in users {
        let selected_communities: Vec<_> = communities.iter().choose_multiple(&mut rng, count);

        for community_id in selected_communities {
            let result = sqlx::query!(
                r#"
                INSERT INTO "user-community" ("user_id", "community_id")
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                user_id,
                community_id
            )
            .execute(pool)
            .await?;

            if result.rows_affected() > 0 {
                inserted_pairs.push((user_id, *community_id));
            }
        }
    }

    Ok(inserted_pairs)
}

async fn seed_manager_community(
    pool: &PgPool,
    managers: &Vec<Uuid>,
    communities: &Vec<Uuid>,
    count: usize,
) -> anyhow::Result<Vec<(Uuid, Uuid)>> {
    let mut rng = rand::rng();
    let mut inserted_pairs = Vec::new();

    for &manager_id in managers {
        let selected_communities: Vec<_> = communities.iter().choose_multiple(&mut rng, count);

        for community_id in selected_communities {
            let result = sqlx::query!(
                r#"
                INSERT INTO "manager-community" ("manager_id", "community_id")
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                manager_id,
                community_id
            )
            .execute(pool)
            .await?;

            if result.rows_affected() > 0 {
                inserted_pairs.push((manager_id, *community_id));
            }
        }
    }

    Ok(inserted_pairs)
}
