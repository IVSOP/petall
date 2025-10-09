use crate::AppState;
use crate::models::{Community, EnergyRecord, UserCommunity, UserRole};
use crate::router::OrderDirection;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use sqlx::Row;
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedEnergyRecords {
    pub records: Vec<EnergyRecord>,
    pub total_count: i64,
}

impl AppState {
    pub async fn get_communities_from_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Vec<(Community, UserRole)>> {
        let rows = sqlx::query!(
            r#"
            SELECT c.id, c.name, c.description, c.image, uc.role as "role: UserRole" FROM community c
            JOIN user_community uc ON c.id = uc.community_id
            WHERE uc.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    Community {
                        id: row.id,
                        name: row.name,
                        description: row.description,
                        image: row.image,
                    },
                    row.role,
                )
            })
            .collect())
    }

    pub async fn create_community(
        &self,
        creator_user_id: Uuid,
        name: &str,
        description: &str,
        image: Option<&str>,
    ) -> sqlx::Result<Community> {
        let mut tx = self.pg_pool.begin().await?;

        let community = sqlx::query_as!(
            Community,
            r#"
            INSERT INTO community
            (name, description, image)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            name,
            description,
            image
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO user_community
            (user_id, community_id, role)
            VALUES ($1, $2, $3)
            "#,
            creator_user_id,
            community.id,
            &UserRole::Manager as &UserRole
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        ///////////////////////////////////////////////////////////////
        let energy_min = BigDecimal::from_f64(0.1).unwrap();
        let energy_max = BigDecimal::from(5000);
        let energy_range = energy_min..energy_max;

        let price_min = BigDecimal::from_f64(0.1).unwrap();
        let price_max = BigDecimal::from(20);
        let price_range = price_min..price_max;

        // enery records are 3.5 months into the past
        let end = Utc::now().naive_utc();
        let days: f32 = 30.0 * 3.5;
        let start = end - Duration::days(days.floor() as i64);
        let date_range = start..end;

        let random_records = EnergyRecord::random_vec(
            &creator_user_id,
            &community.id,
            date_range,
            energy_range,
            price_range,
        );

        info!(
            "Generated {} new random energy records",
            random_records.len()
        );

        self.add_energy_records(&random_records).await?;
        ///////////////////////////////////////////////////////////////

        Ok(community)
    }

    pub async fn get_community_by_id(
        &self,
        user_id: Uuid,
        id: Uuid,
    ) -> sqlx::Result<Option<(Community, UserRole)>> {
        let rows = sqlx::query!(
            r#"
            SELECT c.id, c.name, c.description, c.image, uc.role as "role: UserRole" FROM community c
            JOIN user_community uc ON c.id = uc.community_id
            WHERE c.id = $1 AND uc.user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(rows.map(|row| {
            (
                Community {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    image: row.image,
                },
                row.role,
            )
        }))
    }

    // FIX: ISTO NUNCA É USADO
    pub async fn register_user_community(
        &self,
        community: &Uuid,
        user: Uuid,
        role: UserRole,
    ) -> sqlx::Result<UserCommunity> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            INSERT INTO user_community
            (community_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING
            community_id, user_id, role as "role: UserRole"
            "#,
            community,
            user,
            &role as &UserRole
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn add_energy_records(&self, records: &Vec<EnergyRecord>) -> sqlx::Result<()> {
        const CHUNK_SIZE: usize = 1000; // estava a chegar a limite de argumentos para a query

        for chunk in records.chunks(CHUNK_SIZE) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO energy_record (user_id, community_id, generated, consumed, consumer_price, seller_price, start) ",
            );

            query_builder.push_values(chunk, |mut b, record| {
                b.push_bind(record.user_id)
                    .push_bind(record.community_id)
                    .push_bind(record.generated.clone())
                    .push_bind(record.consumed.clone())
                    .push_bind(record.consumer_price.clone())
                    .push_bind(record.seller_price.clone())
                    .push_bind(record.start);
            });

            query_builder.build().execute(&self.pg_pool).await?;
        }

        Ok(())
    }

    pub async fn remove_user_community(
        &self,
        community: &Uuid,
        user: &Uuid,
    ) -> sqlx::Result<UserCommunity> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            DELETE FROM user_community
            WHERE community_id = $1 AND user_id = $2
            RETURNING
            community_id, user_id, role as "role: UserRole"
            "#,
            community,
            user
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn get_user_energy_records(
        &self,
        user_id: Uuid,
        community_id: Uuid,
        page: u32,
        size: u32,
        order_dir: OrderDirection,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
    ) -> sqlx::Result<PaginatedEnergyRecords> {
        let mut count_builder = QueryBuilder::new(
            r#"
            SELECT COUNT(*)
            FROM energy_record
            WHERE user_id = "#,
        );

        count_builder.push_bind(user_id);
        count_builder.push(" AND community_id = ");
        count_builder.push_bind(community_id);

        if let Some(start_time) = start {
            count_builder.push(" AND start >= ");
            count_builder.push_bind(start_time);
        }

        if let Some(end_time) = end {
            count_builder.push(" AND start <= ");
            count_builder.push_bind(end_time);
        }

        let total_count: i64 = count_builder.build().fetch_one(&self.pg_pool).await?.get(0);

        let mut query_builder = QueryBuilder::new(
            r#"
            SELECT id, user_id, community_id, generated, consumed, consumer_price, seller_price, start
            FROM energy_record
            WHERE user_id = "#,
        );

        query_builder.push_bind(user_id);
        query_builder.push(" AND community_id = ");
        query_builder.push_bind(community_id);

        if let Some(start_time) = start {
            query_builder.push(" AND start >= ");
            query_builder.push_bind(start_time);
        }

        if let Some(end_time) = end {
            query_builder.push(" AND start <= ");
            query_builder.push_bind(end_time);
        }

        let order_dir = match order_dir {
            OrderDirection::Ascending => "ASC",
            OrderDirection::Descending => "DESC",
        };

        query_builder.push(format!(" ORDER BY start {}", order_dir));
        query_builder.push(format!(" LIMIT {} OFFSET {}", size, (page - 1) * size));

        let records = query_builder
            .build_query_as::<EnergyRecord>()
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(PaginatedEnergyRecords {
            records,
            total_count,
        })
    }
}
