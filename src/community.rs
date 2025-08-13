use crate::router::CommunityRegisterRequest;
use crate::{error::AppResult, AppState};
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub entity: String,
}

pub async fn get_community_from_id(id: Uuid, state: &AppState) -> AppResult<Option<Community>> {
    Ok(
        sqlx::query_as!(Community, "SELECT * FROM community WHERE id = $1", id)
            .fetch_optional(&state.pg_pool)
            .await?,
    )
}

pub async fn register_cominunity(community_request: CommunityRegisterRequest, state: &AppState) -> AppResult<Community> {

    let community = Community {
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
