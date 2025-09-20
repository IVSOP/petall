use crate::AppState;
use crate::community::Community;
use crate::error::{AppError, AppResult};
use axum::extract::Query;
use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/community/{id}", get(get_community))
        .route("/community/register", post(register_community))
        .route(
            "/community/{community_id}/energytransfer/{participant_id}",
            get(get_participant_energytransfer),
        )
        .route("/participant/{participant_id}", get(get_participant))
        .route(
            "/participant/{participant_id}/communities",
            get(get_participant_communities),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
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
pub struct EnergyTransferQuery {
    #[validate(range(min = 1, max = 100))]
    pub size: u32,
    pub order_dir: OrderDirection,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
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
    if state
        .get_community_by_name(&request.name)
        .await?
        .is_some()
    {
        Err(AppError::CommunityNameAlreadyInUse(request.name))
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
        if let Some(community) = state.get_community_by_id(&pc.community).await? {
            communities.push(community);
        } else {
            return Err(AppError::CommunityNotFound(pc.community));
        }
    }

    Ok(Json(communities))
}

#[debug_handler]
pub async fn get_participant_energytransfer(
    State(state): State<AppState>,
    Path((community_id, participant_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<EnergyTransferQuery>,
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
        .get_participant_energytransfer(&participant_id, &community_id, query)
        .await?;

    Ok(Json(energy_transfer))
}
