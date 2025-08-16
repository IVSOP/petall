use crate::AppState;
use crate::router::CommunityRegisterRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub entity: String,
    pub supplier: Uuid,
}

impl AppState {
    pub async fn get_community_by_entity(&self, entity: String) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            WHERE entity = $1
            "#,
            entity
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_community_by_id(&self, id: Uuid) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn register_community(
        &self,
        community_request: CommunityRegisterRequest,
    ) -> sqlx::Result<Community> {
        Ok(
            sqlx::query_as!(
                Community,
                r#"
                INSERT INTO community
                (entity, supplier)
                VALUES ($1, $2)
                RETURNING *
                "#,
                community_request.entity,
                community_request.supplier,
            )
            .fetch_one(&self.pg_pool)
            .await?
        )
    }
}
