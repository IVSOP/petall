use crate::AppState;
use crate::router::CommunityRegisterRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub entity: String,
}

impl AppState {
    pub async fn get_community_by_entity(&self, entity: String) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            "SELECT * FROM community WHERE entity = $1",
            entity
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_community_by_id(&self, id: Uuid) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            "SELECT id, entity FROM community WHERE id = $1",
            id
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn register_community(
        &self,
        community_request: CommunityRegisterRequest,
    ) -> sqlx::Result<Community> {
        // TODO: make the database create this uuid
        let community: Community = Community {
            id: Uuid::new_v4(),
            entity: community_request.entity,
        };

        sqlx::query!(
            "INSERT INTO community (id, entity) VALUES ($1, $2)",
            community.id,
            community.entity,
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(community)
    }
}
