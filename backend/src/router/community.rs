use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::models::http::requests::{
    CommunityRegisterRequest, ParticipantCommunityRegisterRequest,
};
use axum::http::StatusCode;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
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
pub async fn register_community(
    State(state): State<AppState>,
    Json(request): Json<CommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    match state.register_community(&request).await {
        Ok(community) => Ok((StatusCode::CREATED, Json(community))),
        Err(_e) => Err(AppError::CommunityNameAlreadyInUse(request.name)),
    }
}

#[debug_handler]
pub async fn register_participant_community(
    State(state): State<AppState>,
    Path(community_id): Path<Uuid>,
    Json(request): Json<ParticipantCommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    match state
        .register_participant_community(&community_id, &request)
        .await
    {
        Ok(participant_community) => Ok((StatusCode::CREATED, Json(participant_community))),
        Err(_e) => Err(AppError::ParticipantCommunityAlredyInUse(
            request.participant,
            community_id,
        )),
    }
}

#[debug_handler]
pub async fn remove_participant_community(
    State(state): State<AppState>,
    Path((community_id, participant_id)): Path<(Uuid, Uuid)>,
) -> AppResult<impl IntoResponse> {
    match state
        .remove_participant_community(&community_id, &participant_id)
        .await
    {
        Ok(participant_community) => Ok((StatusCode::OK, Json(participant_community))),
        Err(_e) => Err(AppError::ParticipantCommunityNotFound(
            participant_id,
            community_id,
        )),
    }
}
