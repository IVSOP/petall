use crate::AppState;
use crate::router::CommunityRegisterRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub image: Uuid,
}

impl AppState {
    pub async fn get_communities(&self) -> sqlx::Result<Vec<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_community_by_name(
        &self,
        name: &String,
    ) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_community_by_id(&self, id: &Uuid) -> sqlx::Result<Option<Community>> {
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
        sqlx::query_as!(
            Community,
            r#"
            INSERT INTO community
            (name)
            VALUES ($1)
            RETURNING *
            "#,
            community_request.name,
        )
        .fetch_one(&self.pg_pool)
        .await
    }
}
