use crate::{AppState, error::AppResult, models::{AuthProvider, Key}};
use uuid::Uuid;

impl AppState {
    pub async fn create_key(
        &self,
        provider: AuthProvider,
        key_id: &str,
        user_id: Uuid,
        hashed_password: Option<String>,
    ) -> AppResult<Key> {
        sqlx::query_as!(
            Key,
            r#"
            INSERT INTO "key" (provider, id, user_id, hashed_password)
            VALUES ($1, $2, $3, $4)
            RETURNING id, provider as "provider: AuthProvider", user_id, hashed_password
            "#,
            provider as AuthProvider,
            key_id,
            user_id,
            hashed_password
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_key(&self, provider: AuthProvider, key_id: &str) -> AppResult<Option<Key>> {
        sqlx::query_as!(
            Key,
            r#"
            SELECT id, provider as "provider: AuthProvider", user_id, hashed_password FROM "key"
            WHERE provider = $1 AND id = $2
            "#,
            provider as AuthProvider,
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
            SELECT id, provider as "provider: AuthProvider", user_id, hashed_password FROM "key"
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
        provider: AuthProvider,
        key_id: &str,
        hashed_password: String,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE "key"
            SET hashed_password = $3
            WHERE provider = $1 AND id = $2
            "#,
            provider as AuthProvider,
            key_id,
            hashed_password
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn delete_key(&self, provider: AuthProvider, key_id: &str) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM "key"
            WHERE provider = $1 AND id = $2
            "#,
            provider as AuthProvider,
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
