use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{AppState, error::AppResult};

impl AppState {
    pub async fn store_token(
        &self,
        token_id: Uuid,
        user_id: Uuid,
        expiration: DateTime<Utc>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO token (id, user_id, expiration)
            VALUES ($1, $2, $3)
            "#,
            token_id,
            user_id,
            expiration
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn is_token_valid(&self, token_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            SELECT id FROM token
            WHERE id = $1 AND expiration > NOW()
            "#,
            token_id
        )
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn delete_token(&self, token_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM token
            WHERE id = $1
            "#,
            token_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(result.rows_affected() != 0)
    }

    pub async fn delete_all_tokens(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM token
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
