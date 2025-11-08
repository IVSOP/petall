use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::models::{Community, EnergyRecord, User, UserCommunity};
use crate::router::community::{EnergyStats, OrderDirection, StatsFilter, StatsGranularity};
use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Row};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedEnergyRecords {
    pub records: Vec<EnergyRecord>,
    pub total_count: i64,
}

impl AppState {
    pub async fn get_communities_with_user_energy_records(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Vec<(Community, bool)>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                c.id,
                c.name,
                c.description,
                c.image,
                EXISTS (
                    SELECT 1 FROM community_user cu
                    WHERE cu.community_id = c.id
                    AND cu.user_id = $1
                ) as "is_present!"
            FROM community c
            WHERE EXISTS (
                SELECT 1 FROM energy_record er
                WHERE er.community_id = c.id
                AND er.user_id = $1
            )
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
                    row.is_present,
                )
            })
            .collect())
    }

    pub async fn create_community(
        &self,
        name: &str,
        description: &str,
        image: Option<&str>,
    ) -> AppResult<Community> {
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
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::CommunityNameAlreadyInUse(name.to_string())
            }
            other => other.into(),
        })?;

        Ok(community)
    }

    pub async fn get_community_by_id(&self, id: Uuid) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT id, name, description, image FROM community
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_users_from_community(&self, community_id: Uuid) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT u.id, u.name, u.email, u.is_admin FROM "user" u
            JOIN community_user cu ON u.id = cu.user_id
            WHERE cu.community_id = $1
            "#,
            community_id,
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_managers_from_community(&self, community_id: Uuid) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT u.id, u.name, u.email, u.is_admin FROM "user" u
            JOIN community_manager cm ON u.id = cm.user_id
            WHERE cm.community_id = $1
            "#,
            community_id,
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn add_user_to_community(
        &self,
        community_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<UserCommunity> {
        let user = sqlx::query_as!(
            UserCommunity,
            r#"
            INSERT INTO community_user
            (community_id, user_id)
            VALUES ($1, $2)
            RETURNING
            community_id, user_id
            "#,
            community_id,
            user_id,
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::UserAlreadyAddedToCommunity(user_id)
            }
            other => other.into(),
        })?;

        // Only for demonstration purposes
        // This will make data duplicated if the user already has energy records for this community
        let now = Utc::now().naive_utc();
        let start = now - Duration::days(90);

        let random_records = EnergyRecord::random_vec(user_id, community_id, start, now);

        self.insert_energy_records(&random_records).await?;

        Ok(user)
    }

    pub async fn insert_energy_records(&self, records: &Vec<EnergyRecord>) -> sqlx::Result<()> {
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

    pub async fn remove_user_from_community(
        &self,
        community_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM community_user
            WHERE community_id = $1 AND user_id = $2
            "#,
            community_id,
            user_id,
        )
        .execute(&self.pg_pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::ManagerNotInCommunity(user_id));
        }

        Ok(())
    }

    pub async fn add_manager_to_community(
        &self,
        community_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO community_manager
            (community_id, user_id)
            VALUES ($1, $2)
            "#,
            community_id,
            user_id,
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::ManagerAlreadyAddedToCommunity(user_id)
            }
            other => other.into(),
        })?;

        Ok(())
    }

    pub async fn remove_manager_from_community(
        &self,
        community_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM community_manager
            WHERE community_id = $1 AND user_id = $2
            "#,
            community_id,
            user_id,
        )
        .execute(&self.pg_pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::ManagerNotInCommunity(user_id));
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
            DELETE FROM community_user
            WHERE community_id = $1 AND user_id = $2
            RETURNING
            community_id, user_id
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

    // NOTE: if you ask for monthly between the second to last day of september and the second of october,
    // there will be 2 entries, one for each month, but the summed values will only be those of registries contained between the provided start and end
    /*
    SELECT DATE_TRUNC(week, start) AS period_start, SUM(generated) AS generated_sum
    FROM energy_record
    WHERE user_id = a4567ef9-0a28-4692-a3dc-59a201d54ee0 AND community_id = 53d11c36-bd41-4eeb-9088-6efffd16a96f AND start >= 2025-10-10 19:59:44 AND start <= 2025-07-10 19:59:44
    GROUP BY period_start
    ORDER BY period_start DESC
    */
    pub async fn get_energy_records_stats(
        &self,
        user_id: Uuid,
        community_id: Uuid,
        filter: &StatsFilter,
    ) -> sqlx::Result<Vec<EnergyStats>> {
        // Choose date_trunc precision based on granularity
        let date_trunc_unit = match filter.granularity {
            StatsGranularity::All => None,
            StatsGranularity::Daily => Some("day"),
            StatsGranularity::Weekly => Some("week"),
            StatsGranularity::Monthly => Some("month"),
            StatsGranularity::Yearly => Some("year"),
        };

        let mut query_builder = QueryBuilder::new("SELECT ");

        if let Some(unit) = date_trunc_unit {
            query_builder.push("DATE_TRUNC(");
            query_builder.push_bind(unit);
            query_builder.push(", start) AS period_start, ");
        } else {
            query_builder.push("MIN(start) AS period_start, ");
        }

        query_builder.push(
            "SUM(generated) AS generated_sum, \
            SUM(consumed) AS consumed_sum, \
            SUM(generated * seller_price) AS generated_price, \
            SUM(consumed * consumer_price) AS consumed_price ",
        );
        query_builder.push("FROM energy_record WHERE user_id = ");
        query_builder.push_bind(user_id);
        query_builder.push(" AND community_id = ");
        query_builder.push_bind(community_id);
        query_builder.push(" AND start >= ");
        query_builder.push_bind(filter.start);
        query_builder.push(" AND start <= ");
        query_builder.push_bind(filter.end);

        if date_trunc_unit.is_some() {
            query_builder.push(" GROUP BY period_start ORDER BY period_start DESC");
        }

        let results = query_builder
            .build_query_as::<EnergyStats>()
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(results)
    }
}
