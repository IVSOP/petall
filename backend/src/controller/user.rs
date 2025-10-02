use crate::error::AppResult;
use crate::models::db::community::Energy;
use crate::models::db::user::{User, UserCommunity, UserRole};
use crate::models::http::OrderDirection;
use crate::router::user::EnergyQuery;
use crate::{AppState, auth};
use sqlx::QueryBuilder;
use uuid::Uuid;

impl AppState {
    pub async fn get_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE id = $1
            "#,
            user_id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn register_user(&self, email: &str, name: &str, password: &str) -> AppResult<User> {
        let hashed_password = auth::password::hash_password(password)?;

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (email, name, password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            email,
            name,
            hashed_password
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn update_user_password(&self, user_id: &Uuid, new_password: &str) -> AppResult<()> {
        let hashed_password = auth::password::hash_password(new_password)?;

        sqlx::query!(
            r#"
            UPDATE "user"
            SET password = $2
            WHERE id = $1
            "#,
            user_id,
            hashed_password
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_communities(&self, user: &Uuid) -> sqlx::Result<Vec<UserCommunity>> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            SELECT user_id, community_id, role as "role: UserRole" FROM "user_community"
            WHERE user_id = $1
            "#,
            user
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_user_energies(
        &self,
        user_id: &Uuid,
        community_id: &Uuid,
        query: EnergyQuery,
    ) -> sqlx::Result<Vec<Energy>> {
        let mut query_builder = QueryBuilder::new(
            r#"
            SELECT id, user_id, community_id, generated, consumed, consumer_price, seller_price, start
            FROM energypool
            WHERE user_id = "#,
        );

        query_builder.push_bind(user_id);
        query_builder.push(" AND community_id = ");
        query_builder.push_bind(community_id);

        if let Some(start) = query.start {
            query_builder.push(" AND start >= ");
            query_builder.push_bind(start);
        }

        if let Some(end) = query.end {
            query_builder.push(" AND \"end\" <= ");
            query_builder.push_bind(end);
        }

        let order_dir = match query.order_dir {
            OrderDirection::Ascending => "ASC",
            OrderDirection::Descending => "DESC",
        };

        query_builder.push(format!(" ORDER BY start {}", order_dir));
        query_builder.push(format!(
            " LIMIT {} OFFSET {}",
            query.size,
            (query.page - 1) * query.size
        ));

        query_builder
            .build_query_as::<Energy>()
            .fetch_all(&self.pg_pool)
            .await
    }
}
