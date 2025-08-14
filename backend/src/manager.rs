use crate::{error::AppResult, AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manager {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

pub async fn get_manager_by_id(id: Uuid, state: &AppState) -> AppResult<Option<Manager>> {
    Ok(
        sqlx::query_as!(
            Manager,
            r#"SELECT * FROM "manager" WHERE id = $1"#,
            id
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}

pub async fn _get_manager_by_email(email: &str, state: &AppState) -> AppResult<Option<Manager>> {
    Ok(
        sqlx::query_as!(
            Manager,
            r#"SELECT * FROM "manager" WHERE email = $1"#,
            email
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}
