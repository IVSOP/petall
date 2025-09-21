use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::models::http::requests::EnergyQuery;
use crate::models::http::response::ParticipantCommunityResponse;
use uuid::Uuid;
use validator::Validate;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};

#[debug_handler]
pub async fn get_participant(
    State(state): State<AppState>,
    Path(participant_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    match state.get_participant_by_id(&participant_id).await? {
        Some(participant) => Ok((StatusCode::OK, Json(participant))),
        None => Err(AppError::ParticipantNotFoundId(participant_id)),
    }
}

#[debug_handler]
pub async fn get_participants(
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    match state.get_participants().await {
        Ok(participants) => Ok((StatusCode::OK, Json(participants))),
        Err(e) => Err(AppError::from(e)),
    }
}

#[debug_handler]
pub async fn get_participant_communities(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let mut communities: Vec<ParticipantCommunityResponse> = Vec::new();
    let participant_communities = state.get_participant_communities(&id).await?;

    for pc in &participant_communities {
        match state.get_community_by_id(&pc.community).await? {
            Some(com) => communities.push(ParticipantCommunityResponse {
                community: com,
                role: pc.role,
            }),
            None => return Err(AppError::CommunityNotFound(pc.community)),
        }
    }

    Ok(Json(communities))
}

#[debug_handler]
pub async fn get_participant_energies(
    State(state): State<AppState>,
    Path((community_id, participant_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<EnergyQuery>,
) -> AppResult<impl IntoResponse> {
    query.validate()?;
    // TODO: Change this to a EXISTS query
    if state
        .get_participant_by_id(&participant_id)
        .await?
        .is_none()
    {
        return Err(AppError::ParticipantNotFoundId(participant_id));
    }

    if state.get_community_by_id(&community_id).await?.is_none() {
        return Err(AppError::CommunityNotFound(community_id));
    }

    let energy_transfer = state
        .get_participant_energies(&participant_id, &community_id, query)
        .await?;

    Ok(Json(energy_transfer))
}