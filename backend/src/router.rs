use crate::AppState;
use crate::community::Community;
use crate::error::{AppError, AppResult};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, delete},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/communities",
            get(get_communities))
        .route(
            "/community/{id}",
            get(get_community))
        .route(
            "/community/register",
            post(register_community))
        .route(
            "/community/{community_id}/participant",
            post(register_participant_community),
        )
        .route(
            "/community/{community_id}/participant/{participant_id}",
            delete(remove_participant_community),
        )
        .route(
            "/community/{community_id}/energy/{participant_id}",
            get(get_participant_energies),
        )
        .route(
            "/participants",
            get(get_participants))
        .route(
            "/participant/{participant_id}",
            get(get_participant))
        .route(
            "/participant/{participant_id}/communities",
            get(get_participant_communities),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "lowercase")]
pub enum ParticipantRole {
    User,
    Manager,
    UserManager,
}

#[derive(Deserialize, Default)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    #[default]
    Descending,
}

#[derive(Deserialize, Validate)]
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

#[derive(Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct ParticipantCommunityRegisterRequest {
    pub participant: Uuid,
    pub role: ParticipantRole,
}

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
pub async fn get_communities(
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    match state.get_communities().await {
        Ok(communities) => Ok((StatusCode::OK, Json(communities))),
        Err(e) => Err(AppError::from(e)),
    }
}

#[debug_handler]
async fn register_community(
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
async fn register_participant_community(
    State(state): State<AppState>,
    Path(community_id): Path<Uuid>,
    Json(request): Json<ParticipantCommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    let participant_community = state.register_participant_community(&community_id, &request).await?;
    Ok((StatusCode::CREATED, Json(participant_community)))
}

#[debug_handler]
async fn remove_participant_community(
    State(state): State<AppState>,
    Path((community_id, participant_id)): Path<(Uuid, Uuid)>,
) -> AppResult<impl IntoResponse> {
    let deleted = state
        .remove_participant_community(&community_id, &participant_id)
        .await?;
    if deleted.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::ParticipantCommunityNotFound(participant_id, community_id))
    }
}

#[debug_handler]
pub async fn get_participant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(participant) = state.get_participant_by_id(&id).await? {
        Ok(Json(participant))
    } else {
        Err(AppError::ParticipantNotFoundId(id))
    }
}

#[debug_handler]
pub async fn get_participants(
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let participants = state.get_participants().await?;
    Ok(Json(participants))
}


#[debug_handler]
pub async fn get_participant_communities(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if state.get_participant_by_id(&id).await?.is_none() {
        return Err(AppError::ParticipantNotFoundId(id));
    }

    let mut communities: Vec<Community> = Vec::new();
    let participant_communities = state.get_participant_communities(&id).await?;

    for pc in &participant_communities {
        if let Some(community) = state.get_community_by_id(&pc.community).await? {
            communities.push(community);
        } else {
            return Err(AppError::CommunityNotFound(pc.community));
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
