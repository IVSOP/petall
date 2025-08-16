use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use fake::{Fake, faker::internet::en::FreeEmail};
use names::Generator;
use rand::seq::SliceRandom;
use rand::{Rng, seq::IteratorRandom};
use sqlx::postgres::PgPool;
use std::str::FromStr;
use std::collections::HashMap;
use uuid::Uuid;
use crate::participant::ParticipantRole;

#[derive(Debug, Clone, clap::Parser)]
pub struct SeedSettings {
    #[arg(long, default_value_t = 5)]
    users: usize,
    #[arg(long, default_value_t = 5)]
    managers: usize,
    #[arg(long, default_value_t = 3)]
    suppliers: usize,
    #[arg(long, default_value_t = 3)]
    communities_per_supplier: usize,
    #[arg(long, default_value_t = 5)]
    communities_per_user: usize,
    #[arg(long, default_value_t = 5)]
    communities_per_manager: usize,
}

pub async fn run_seed(pg_pool: &PgPool, seed_settings: SeedSettings) -> anyhow::Result<()> {
    let users_ids = seed_participant(
        pg_pool,
        &ParticipantRole::User,
        &seed_settings.users,
    ).await?;

    let manager_ids = seed_participant(
        pg_pool,
        &ParticipantRole::Manager,
        &seed_settings.managers,
    ).await?;

    let supplier_ids = seed_participant(
        pg_pool,
        &ParticipantRole::Supplier,
        &seed_settings.suppliers,
    ).await?;

    let supplier_communities_map = seed_community(
        pg_pool,
        &supplier_ids,
        &seed_settings.communities_per_supplier,
    ).await?;

    let community_ids: Vec<Uuid> = supplier_communities_map.values().flatten().cloned().collect();

    let mut community_users_map = seed_participant_community(
        pg_pool,
        &users_ids,
        &community_ids,
        &seed_settings.communities_per_user,
    ).await?;

    seed_participant_community(
        pg_pool,
        &manager_ids,
        &community_ids,
        &seed_settings.communities_per_manager,
    ).await?;

    seed_energytransfer(
        pg_pool,
        &supplier_communities_map,
        &mut community_users_map,
    ).await?;

    Ok(())
}

pub async fn seed_participant(
    pool: &PgPool,
    role: &ParticipantRole,
    count: &usize,
) -> anyhow::Result<Vec<Uuid>> {
    let mut generator = Generator::default();
    let mut participant_ids = Vec::new();

    for _ in 0..*count {
        let name = generator.next().unwrap();
        let email = FreeEmail().fake::<String>();
        participant_ids.push(
            sqlx::query_scalar!(
                r#"
                INSERT INTO "participant" ("role", "name", "email")
                VALUES ($1, $2, $3)
                RETURNING id
                "#,
                role as &ParticipantRole,
                name,
                email
            )
            .fetch_one(pool)
            .await?
        )
    }

    Ok(participant_ids)
}


pub async fn seed_community(
    pool: &PgPool,
    suppliers: &Vec<Uuid>,
    count: &usize,
) -> anyhow::Result<HashMap<Uuid, Vec<Uuid>>> {
    let mut generator = Generator::default();
    let mut supplier_communities_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for &supplier in suppliers {
        for _ in 0..*count {
            let entity = generator.next().unwrap();
            let community_id = sqlx::query_scalar!(
                r#"
                INSERT INTO "community" ("entity", "supplier")
                VALUES ($1, $2)
                RETURNING id
                "#,
                entity,
                supplier
            )
            .fetch_one(pool)
            .await?;

            supplier_communities_map
                .entry(supplier)
                .or_default()
                .push(community_id);
        }
    }

    Ok(supplier_communities_map)
}


pub async fn seed_participant_community(
    pool: &PgPool,
    participants: &Vec<Uuid>,
    communities: &Vec<Uuid>,
    count: &usize,
) -> anyhow::Result<HashMap<Uuid, Vec<Uuid>>> {  
    let mut rng = rand::rng();
    let mut community_participant_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for &participant_id in participants {
        for community_id in communities.iter().choose_multiple(&mut rng, *count) {
            sqlx::query!(
                r#"
                INSERT INTO "participant_community" ("participant_id", "community_id")
                VALUES ($1, $2)
                "#,
                participant_id,
                community_id,
            )
            .execute(pool)
            .await?;

            community_participant_map
                .entry(*community_id)
                .or_default()
                .push(participant_id);
        }
    }

    Ok(community_participant_map)
}

pub async fn seed_energytransfer(
    pool: &PgPool,
    supplier_communities: &HashMap<Uuid, Vec<Uuid>>,
    community_participant_map: &mut HashMap<Uuid, Vec<Uuid>>,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut community_supplier: HashMap<Uuid, Uuid> = HashMap::new();

    let start = Utc::now().naive_utc();
    let end = (Utc::now() + Duration::days(1)).naive_utc();

    for (k, v) in supplier_communities {
        for community in v {
            community_supplier.entry(*community).or_insert(*k);
        }
    }

    for (community, participants) in community_participant_map.iter_mut() {
        let mut current = start;

        while current < end {
            participants.shuffle(&mut rng);

            for chunk in participants.chunks(2) {
                let mut rng = rand::rng();
                let energy_wh = BigDecimal::from_str(
                    &rng.random_range(10.0..5000.0).to_string()
                ).unwrap();

                let participant_to = if chunk.len() == 2 {
                   chunk[1]
                } else {
                    *community_supplier.get(community)
                        .expect("No supplier found for community")
                };

                sqlx::query!(
                    r#"
                    INSERT INTO "energytransfer" ("id", "participant_from", "participant_to", "community", "energy_wh", "start", "end")
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT DO NOTHING
                    "#,
                    Uuid::new_v4(),
                    chunk[0],
                    participant_to,
                    community,
                    energy_wh,
                    current,
                    current + Duration::minutes(15),
                )
                .execute(pool)
                .await
                .unwrap();
            }
            current += Duration::minutes(15);
        }
    }

    Ok(())
}
