use crate::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCommunity {
    pub user_id: Uuid,
    pub community_id: Uuid,
}

impl AppState {
    // WARN: vec may be empty
    pub async fn get_communities_by_user(&self, user_id: Uuid) -> sqlx::Result<Vec<UserCommunity>> {
        sqlx::query_as!(
            UserCommunity,
            r#"SELECT * FROM "user-community" WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
