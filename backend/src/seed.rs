use crate::models::UserRole;
use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use fake::{Fake, faker::internet::pt_pt::FreeEmail};
use names::Generator;
use rand::{Rng, seq::IteratorRandom};
use sqlx::postgres::PgPool;
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, clap::Parser)]
pub struct SeedSettings {
    #[arg(long, default_value_t = 5)]
    users: usize,
    #[arg(long, default_value_t = 3)]
    suppliers: usize,
    #[arg(long, default_value_t = 10)]
    communities: usize,
    #[arg(long, default_value_t = 5)]
    communities_per_user: usize,
    #[arg(long, default_value_t = 1)]
    energy_days: i64,
    #[arg(long, default_value_t = 15)]
    energy_interval: i64,
}

pub async fn run_seed(pg_pool: &PgPool, seed_settings: SeedSettings) -> anyhow::Result<()> {
    // let suppliers = seed_supplier(pg_pool, &seed_settings.suppliers).await?;

    // let users = seed_user(pg_pool, &seed_settings.users, &suppliers).await?;
    let users = seed_user(pg_pool, &seed_settings.users).await?;

    let communitied = seed_community(pg_pool, &seed_settings.communities).await?;

    let user_communities_map = seed_user_community(
        pg_pool,
        &seed_settings.communities_per_user,
        &users,
        &communitied,
    )
    .await?;

    seed_energy_record(
        pg_pool,
        &seed_settings.energy_days,
        &seed_settings.energy_interval,
        &user_communities_map,
    )
    .await?;

    Ok(())
}

// pub async fn seed_supplier(pool: &PgPool, count: &usize) -> anyhow::Result<Vec<Uuid>> {
//     let mut generator = Generator::default();
//     let mut suppliers = Vec::new();

//     for _ in 0..*count {
//         suppliers.push(
//             sqlx::query_scalar!(
//                 r#"
//                 INSERT INTO "supplier" ("email", "name")
//                 VALUES ($1, $2)
//                 RETURNING id
//                 "#,
//                 FreeEmail().fake::<String>(),
//                 generator.next().unwrap()
//             )
//             .fetch_one(pool)
//             .await?,
//         )
//     }

//     Ok(suppliers)
// }

pub async fn seed_user(
    pool: &PgPool,
    count: &usize,
    // suppliers: &[Uuid],
) -> anyhow::Result<Vec<Uuid>> {
    // let mut rng = rand::rng();
    let mut generator = Generator::default();
    let mut users = Vec::new();

    for _ in 0..*count {
        users.push(
            // sqlx::query_scalar!(
            //     r#"
            //     INSERT INTO "user" ("email", "name", "supplier", "password")
            //     VALUES ($1, $2, $3, $4)
            //     RETURNING id
            //     "#,
            //     FreeEmail().fake::<String>(),
            //     generator.next().unwrap(),
            //     suppliers.iter().choose(&mut rng).unwrap(),
            //     "password"
            // )
            sqlx::query_scalar!(
                r#"
                INSERT INTO "user" ("email", "name", "password")
                VALUES ($1, $2, $3)
                RETURNING id
                "#,
                FreeEmail().fake::<String>(),
                generator.next().unwrap(),
                "password"
            )
            .fetch_one(pool)
            .await?,
        )
    }

    Ok(users)
}

pub async fn seed_community(pool: &PgPool, count: &usize) -> anyhow::Result<Vec<Uuid>> {
    let mut generator = Generator::default();
    let mut communities = Vec::new();

    for _ in 0..*count {
        communities.push(
            sqlx::query_scalar!(
                r#"
                INSERT INTO "community" ("name")
                VALUES ($1)
                RETURNING id
                "#,
                generator.next().unwrap(),
            )
            .fetch_one(pool)
            .await?,
        )
    }

    Ok(communities)
}

pub async fn seed_user_community(
    pool: &PgPool,
    communities_per_user: &usize,
    users: &[Uuid],
    communities: &[Uuid],
) -> anyhow::Result<HashMap<Uuid, Vec<Uuid>>> {
    let mut rng = rand::thread_rng();
    let mut user_community_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    let roles = [
        UserRole::Participant,
        UserRole::Manager,
        UserRole::UserManager,
    ];

    for &user in users {
        for community in communities
            .iter()
            .choose_multiple(&mut rng, *communities_per_user)
        {
            sqlx::query!(
                r#"
                INSERT INTO "user_community" ("user_id", "community_id", "role")
                VALUES ($1, $2, $3)
                "#,
                user,
                community,
                roles.iter().choose(&mut rng).unwrap() as &UserRole
            )
            .execute(pool)
            .await?;

            user_community_map.entry(user).or_default().push(*community);
        }
    }

    Ok(user_community_map)
}

pub async fn seed_energy_record(
    pool: &PgPool,
    energy_days: &i64,
    energy_interval: &i64,
    user_communities_map: &HashMap<Uuid, Vec<Uuid>>,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let start = Utc::now().naive_utc();
    let end = (Utc::now() + Duration::days(*energy_days)).naive_utc();

    for (user, communities) in user_communities_map.iter() {
        let mut current = start;
        while current < end {
            for community in communities {
                sqlx::query!(
                    r#"
                    INSERT INTO "energy_record" ("user_id", "community_id", "generated", "consumed", "consumer_price", "seller_price", "start")
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                    user,
                    community,
                    BigDecimal::from_str(&rng.gen_range(0.0..5000.0).to_string()).unwrap(),
                    BigDecimal::from_str(&rng.gen_range(0.0..5000.0).to_string()).unwrap(),
                    BigDecimal::from_str(&rng.gen_range(0.0..20.0).to_string()).unwrap(),
                    BigDecimal::from_str(&rng.gen_range(0.0..20.0).to_string()).unwrap(),
                    current,
                )
                .execute(pool)
                .await
                .unwrap();
            }
            current += Duration::minutes(*energy_interval);
        }
    }

    Ok(())
}
