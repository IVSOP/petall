use crate::AppState;
use crate::community::Community;
use crate::error::{AppError, AppResult};
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
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(serde::Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub entity: String,
    pub supplier: Uuid,
}

#[derive(serde::Serialize)]
struct CommunityRegisterResponse {
    id: Uuid,
    entity: String,
}

#[debug_handler]
pub async fn get_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(community) = state.get_community_by_id(id).await? {
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

    if let Some(_) = state
        .get_community_by_entity(request.entity.clone())
        .await?
    {
        // entity already exists
        return Err(AppError::CommunityEntityAlreadyInUse(request.entity));
    }

    let community = state.register_community(request).await?;

    Ok(Json(CommunityRegisterResponse {
        id: community.id,
        entity: community.entity,
    }))
}

#[debug_handler]
pub async fn get_participant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(participant) = state.get_participant_by_id(id).await? {
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
    // TODO: validate that the user exists?

    let participant_communities = state.get_participant_communities(id).await?;

    let mut res: Vec<Community> = Vec::new();

    for participant_community in participant_communities.iter() {
        if let Some(community) = state
            .get_community_by_id(participant_community.community_id)
            .await?
        {
            res.push(community);
        } else {
            // FIX: O que fazer quando há erro a ir buscar a comunidade??
        }
    }

    Ok(Json(res))
}
