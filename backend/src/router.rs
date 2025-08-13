use crate::error::{AppResult, AppError};
use crate::{community, AppState};
use axum::{extract::{State, Path}, response::IntoResponse, routing::{get, post}, Json, Router, debug_handler};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;


pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/community/{id}", get(get_community))
        .route("/community/register", post(register_community))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}


#[derive(serde::Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub entity: String,
}

#[derive(serde::Serialize)]
struct CommunityRegisterResponse {
    id: Uuid,
    entity: String,
}

pub async fn get_community(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(community) = community::get_community_by_id(id, &state).await? {
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

    if community::get_community_by_entity(request.entity.clone(), &state)
        .await?
        .is_some()
    {
        // entity already exists
        return Err(AppError::CommunityEntityAlreadyInUse(request.entity));
    }

    let community = community::register_community(request, &state).await?;

    Ok((
        Json(CommunityRegisterResponse {
            id: community.id,
            entity: community.entity,
        }),
    ))
}
