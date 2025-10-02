use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::models::db::user::{User, UserRole};
use axum::http::StatusCode;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[debug_handler]
pub async fn get_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    match state.get_community_by_id(&id).await? {
        Some(community) => Ok((StatusCode::OK, Json(community))),
        None => Err(AppError::CommunityNotFound(id)),
    }
}

#[debug_handler]
pub async fn get_communities(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    match state.get_communities().await {
        Ok(communities) => Ok((StatusCode::OK, Json(communities))),
        Err(e) => Err(AppError::from(e)),
    }
}

#[debug_handler]
pub async fn get_community_users(
    State(state): State<AppState>,
    Path(community_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let mut users: Vec<User> = Vec::new();
    let user_communities = state.get_community_users(&community_id).await?;

    for pc in &user_communities {
        match state.get_user_by_id(&pc.user_id).await? {
            Some(user) => users.push(user),
            None => return Err(AppError::UserNotFoundId(pc.user_id)),
        }
    }

    Ok(Json(users))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

#[debug_handler]
pub async fn register_community(
    State(state): State<AppState>,
    Json(request): Json<CommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    match state.register_community(&request.name).await {
        Ok(community) => Ok((StatusCode::CREATED, Json(community))),
        Err(_e) => Err(AppError::CommunityNameAlreadyInUse(request.name)),
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserCommunityRegisterRequest {
    pub user: Uuid,
    pub role: UserRole,
}

#[debug_handler]
pub async fn register_user_community(
    State(state): State<AppState>,
    Path(community_id): Path<Uuid>,
    Json(request): Json<UserCommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    match state
        .register_user_community(&community_id, request.user, request.role)
        .await
    {
        Ok(user_community) => Ok((StatusCode::CREATED, Json(user_community))),
        Err(_e) => Err(AppError::UserCommunityAlreadyInUse(
            request.user,
            community_id,
        )),
    }
}

#[debug_handler]
pub async fn remove_user_community(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(Uuid, Uuid)>,
) -> AppResult<impl IntoResponse> {
    match state.remove_user_community(&community_id, &user_id).await {
        Ok(user_community) => Ok((StatusCode::OK, Json(user_community))),
        Err(_e) => Err(AppError::UserCommunityNotFound(user_id, community_id)),
    }
}
