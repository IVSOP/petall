use crate::{AppState, error::AppResult, models::Key};
use uuid::Uuid;

impl AppState {
    pub async fn create_key(
        &self,
        key_id: &str,
        user_id: Uuid,
        hashed_password: Option<String>,
    ) -> AppResult<Key> {
        sqlx::query_as!(
            Key,
            r#"
            INSERT INTO "key" (id, user_id, hashed_password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            key_id,
            user_id,
            hashed_password
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_key(&self, key_id: &str) -> AppResult<Option<Key>> {
        sqlx::query_as!(
            Key,
            r#"
            SELECT * FROM "key"
            WHERE id = $1
            "#,
            key_id
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_keys(&self, user_id: Uuid) -> AppResult<Vec<Key>> {
        sqlx::query_as!(
            Key,
            r#"
            SELECT * FROM "key"
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn update_key_password(
        &self,
        key_id: &str,
        hashed_password: String,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE "key"
            SET hashed_password = $2
            WHERE id = $1
            "#,
            key_id,
            hashed_password
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn delete_key(&self, key_id: &str) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM "key"
            WHERE id = $1
            "#,
            key_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_all_user_keys(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM "key"
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
