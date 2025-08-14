use crate::community::Community;
use crate::error::{AppResult, AppError};
use crate::{community, manager, user, user_community, AppState};
use axum::{extract::{State, Path}, response::IntoResponse, routing::{get, post}, Json, Router, debug_handler};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/community/{id}", get(get_community))
        .route("/community/register", post(register_community))
        .route("/user/{user_id}", get(get_user))
        .route("/user/communities/{user_id}", get(get_user_communities))
        .route("/manager/{manager_id}", get(get_manager))
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

#[debug_handler]
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

#[debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(user) = user::get_user_by_id(id, &state).await? {
        Ok(Json(user))
    } else {
        Err(AppError::UserNotFoundId(id))
    }
}

// #[debug_handler]
// pub async fn get_user_from_email(
//     State(state): State<AppState>,
//     Path(email): Path<String>,
// ) -> AppResult<impl IntoResponse> {
//     if let Some(user) = user::get_user_by_email(&email, &state).await? {
//         Ok(Json(user))
//     } else {
//         Err(AppError::UserNotFoundEmail(email))
//     }
// }

#[debug_handler]
pub async fn get_user_communities(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {

    // TODO: validate that the user exists?

    let user_communities = user_community::get_communities_by_user(id, &state).await?;

    let mut res: Vec<Community> = Vec::new();

    for user_community in user_communities.iter() {
        if let Some(community) = community::get_community_by_id(user_community.community_id, &state).await? {            
            res.push(community);
        } else {
            // FIX: O que fazer quando há erro a ir buscar a comunidade??
        }
    }

    Ok(Json(res))
}

#[debug_handler]
pub async fn get_manager(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    if let Some(manager) = manager::get_manager_by_id(id, &state).await? {
        Ok(Json(manager))
    } else {
        Err(AppError::ManagerNotFoundId(id))
    }
}
