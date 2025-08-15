use crate::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManagerCommunity {
    pub manager_id: Uuid,
    pub community_id: Uuid,
}

impl AppState {
    // WARN: vec may be empty
    pub async fn get_communities_by_manager(
        &self,
        manager_id: Uuid,
    ) -> sqlx::Result<Vec<ManagerCommunity>> {
        sqlx::query_as!(
            ManagerCommunity,
            r#"SELECT * FROM "manager-community" WHERE manager_id = $1"#,
            manager_id
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
