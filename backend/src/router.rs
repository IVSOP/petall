use crate::AppState;
use crate::error::{AppError, AppResult};
use crate::community::Community;
use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/community/{id}", get(get_community))
        .route("/community/register", post(register_community))
        .route("/participant/{participant_id}", get(get_participant))
        .route("/participant/communities/{participant_id}", get(get_participant_communities))
        .route("/energytransfer/{community_id}/{participant_id}", get(get_participant_energytransfer))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(serde::Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub entity: String,
    pub supplier: Uuid,
}

#[derive(serde::Deserialize, Validate)]
pub struct EnergyTransferQuery {
    #[validate(range(min = 1))]
    pub page: u32,
}


#[debug_handler]
pub async fn get_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(community) = state.get_community_by_id(&id).await? {
        Ok(Json(community))
    } else {
        Err(AppError::CommunityNotFound(id))
    }
}

#[debug_handler]
async fn register_community(
    State(state): State<AppState>,
    Json(request): Json<CommunityRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;
    if state.get_community_by_entity(&request.entity).await?.is_some() {
        Err(AppError::CommunityEntityAlreadyInUse(request.entity))
    } else {
        Ok(Json(state.register_community(request).await?))
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
        if let Some(community) = state
            .get_community_by_id(&pc.community_id)
            .await?
        {
            communities.push(community);
        } else {
            return Err(AppError::CommunityNotFound(pc.community_id));
        }
    }

    Ok(Json(communities))
}


#[debug_handler]
pub async fn get_participant_energytransfer(
    State(state): State<AppState>,
    Path((community_id, participant_id)): Path<(Uuid, Uuid)>,
    Json(query): Json<EnergyTransferQuery>,
) -> AppResult<impl IntoResponse> {
    query.validate()?;
    if state.get_participant_by_id(&participant_id).await?.is_none() {
        return Err(AppError::ParticipantNotFoundId(participant_id));
    } 

    if state.get_community_by_id(&community_id).await?.is_none() {
        return Err(AppError::CommunityNotFound(community_id));
    }

    Ok(Json(
        state.get_participant_energytransfer(
            &participant_id,
            &community_id,
            query
        ).await?
    ))
}