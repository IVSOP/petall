use crate::{error::AppResult, AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManagerCommunity {
    pub manager_id: Uuid,
    pub community_id: Uuid,
}

// WARN: vec may be empty
pub async fn get_communities_by_manager(manager_id: Uuid, state: &AppState) -> AppResult<Vec<ManagerCommunity>> {
    Ok(
        sqlx::query_as!(
            ManagerCommunity,
            r#"SELECT * FROM "manager-community" WHERE manager_id = $1"#,
            manager_id
        )
        .fetch_all(&state.pg_pool)
        .await?,
    )
}
