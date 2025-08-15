use crate::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl AppState {
    pub async fn get_user_by_id(&self, id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_optional(&self.pg_pool)
            .await
    }

    pub async fn _get_user_by_email(&self, email: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE email = $1"#, email)
            .fetch_optional(&self.pg_pool)
            .await
    }
}
