use crate::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl AppState {
    pub async fn get_manager_by_id(&self, id: Uuid) -> sqlx::Result<Option<Manager>> {
        sqlx::query_as!(Manager, r#"SELECT * FROM "manager" WHERE id = $1"#, id)
            .fetch_optional(&self.pg_pool)
            .await
    }

    pub async fn _get_manager_by_email(&self, email: &str) -> sqlx::Result<Option<Manager>> {
        sqlx::query_as!(
            Manager,
            r#"SELECT * FROM "manager" WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pg_pool)
        .await
    }
}
