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
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn default_order_by() -> String { "start".to_string() }
fn default_order_dir() -> String { "ASC".to_string() }

fn validate_order_dir(value: &str) -> Result<(), ValidationError> {
    match value {
        "ASC" | "DESC" => Ok(()),
        _ => Err(ValidationError::new("invalid_order_dir")),
    }
}

fn validate_order_by(value: &str) -> Result<(), ValidationError> {
    match value {
        "start" | "end" | "energy_wh" => Ok(()),
        _ => Err(ValidationError::new("invalid_order_dir")),
    }
}

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
    #[validate(range(min = 1))]
    pub size: u32,
    #[serde(default = "default_order_by")]
    #[validate(custom(function = "validate_order_by"))]
    pub order_by: String,
    #[serde(default = "default_order_dir")]
    #[validate(custom(function = "validate_order_dir"))]
    pub order_dir: String, 
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
        .get_community_by_entity(&request.entity)
        .await?
        .is_some()
    {
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
        if let Some(community) = state.get_community_by_id(&pc.community_id).await? {
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
    Query(query): Query<EnergyTransferQuery>,
) -> AppResult<impl IntoResponse> {
    query.validate()?;
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

    Ok(Json(
        state
            .get_participant_energytransfer(&participant_id, &community_id, query)
            .await?,
    ))
}
