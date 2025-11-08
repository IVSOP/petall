use uuid::Uuid;

use crate::{AppState, auth::Session, error::AppResult};

impl AppState {
    pub async fn store_session(&self, session: &Session) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO session (id, user_id, expiration)
            VALUES ($1, $2, $3)
            "#,
            session.id,
            session.user_id,
            session.expiration
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn get_valid_session(&self, session_id: Uuid) -> AppResult<Option<Session>> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, expiration FROM session
            WHERE id = $1 AND expiration > NOW()
            "#,
            session_id
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn delete_session(&self, session_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM session
            WHERE id = $1
            "#,
            session_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(result.rows_affected() != 0)
    }

    pub async fn delete_all_sessions(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM session
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
