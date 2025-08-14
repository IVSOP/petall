use crate::{error::AppResult, AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCommunity {
    pub user_id: Uuid,
    pub community_id: Uuid,
}

// WARN: vec may be empty
pub async fn get_communities_by_user(user_id: Uuid, state: &AppState) -> AppResult<Vec<UserCommunity>> {
    Ok(
        sqlx::query_as!(
            UserCommunity,
            r#"SELECT * FROM "user-community" WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&state.pg_pool)
        .await?,
    )
}
