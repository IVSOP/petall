use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{AppState, error::AppResult};

impl AppState {
    pub async fn store_token(
        &self,
        token_id: Uuid,
        participant_id: Uuid,
        expiration: DateTime<Utc>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO token (id, participant, expiration)
            VALUES ($1, $2, $3)
            "#,
            token_id,
            participant_id,
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

    pub async fn delete_all_tokens(&self, participant_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM token
            WHERE participant = $1
            "#,
            participant_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
