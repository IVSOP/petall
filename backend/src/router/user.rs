use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::models::db::community::Community;
use crate::models::db::user::UserRole;
use crate::models::http::OrderDirection;
use axum::Router;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/user/{user_id}", get(get_user))
        .route("/user/{user_id}/communities", get(get_user_communities))
        .route(
            "/community/{community_id}/energy/{user_id}",
            get(get_user_energies),
        )
}

#[debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    match state.get_user_by_id(user_id).await? {
        Some(user) => Ok((StatusCode::OK, Json(user))),
        None => Err(AppError::UserNotFoundId(user_id)),
    }
}

#[debug_handler]
pub async fn get_users(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    match state.get_users().await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(e) => Err(AppError::from(e)),
    }
}

#[derive(Debug, Serialize)]
pub struct UserCommunityResponse {
    pub community: Community,
    pub role: UserRole,
}

#[debug_handler]
pub async fn get_user_communities(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let mut communities: Vec<UserCommunityResponse> = Vec::new();
    let user_communities = state.get_user_communities(&user_id).await?;

    for pc in &user_communities {
        match state.get_community_by_id(&pc.community_id).await? {
            Some(com) => communities.push(UserCommunityResponse {
                community: com,
                role: pc.role,
            }),
            None => return Err(AppError::CommunityNotFound(pc.community_id)),
        }
    }

    Ok(Json(communities))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnergyQuery {
    #[validate(range(min = 1, max = 100))]
    pub page: u32,
    #[validate(range(min = 1, max = 100))]
    pub size: u32,
    pub order_dir: OrderDirection,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

#[debug_handler]
pub async fn get_user_energies(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<EnergyQuery>,
) -> AppResult<impl IntoResponse> {
    query.validate()?;
    // TODO: Change this to a EXISTS query
    if state.get_user_by_id(user_id).await?.is_none() {
        return Err(AppError::UserNotFoundId(user_id));
    }

    if state.get_community_by_id(&community_id).await?.is_none() {
        return Err(AppError::CommunityNotFound(community_id));
    }

    let energy_transfer = state
        .get_user_energies(&user_id, &community_id, query)
        .await?;

    Ok(Json(energy_transfer))
}
