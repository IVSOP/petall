use crate::{error::AppResult, AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

pub async fn get_user_by_id(id: Uuid, state: &AppState) -> AppResult<Option<User>> {
    Ok(
        sqlx::query_as!(
            User,
            r#"SELECT * FROM "user" WHERE id = $1"#,
            id
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}

pub async fn _get_user_by_email(email: &str, state: &AppState) -> AppResult<Option<User>> {
    Ok(
        sqlx::query_as!(
            User,
            r#"SELECT * FROM "user" WHERE email = $1"#,
            email
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}
