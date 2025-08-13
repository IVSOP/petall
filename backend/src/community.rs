use crate::router::CommunityRegisterRequest;
use crate::{error::AppResult, AppState};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub entity: String,
}

pub async fn get_community_by_entity(entity: String, state: &AppState) -> AppResult<Option<Community>> {
    Ok(
        sqlx::query_as!(
            Community,
            "SELECT * FROM community WHERE entity = $1",
            entity
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}

pub async fn get_community_by_id(id: Uuid, state: &AppState) -> AppResult<Option<Community>> {
    Ok(
        sqlx::query_as!(
            Community,
            "SELECT id, entity FROM community WHERE id = $1",
            id
        )
        .fetch_optional(&state.pg_pool)
        .await?,
    )
}

pub async fn register_community(community_request: CommunityRegisterRequest, state: &AppState) -> AppResult<Community> {

    let community: Community = Community {
        id: Uuid::new_v4(),
        entity: community_request.entity,
    };

    sqlx::query!(
        "INSERT INTO community (id, entity) VALUES ($1, $2)",
        community.id,
        community.entity,
    )
    .execute(&state.pg_pool)
    .await?;

    Ok(community)
}
