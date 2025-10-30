use crate::AppState;
use crate::auth::extractor::ExtractSession;
use crate::controller::admin::AdminListCommunityView;
use crate::error::{AppError, AppResult, ValidatedJson};
use crate::models::{Community, User};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Json, debug_handler, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize)]
pub struct AdminCommunityInfoResponseUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

impl From<User> for AdminCommunityInfoResponseUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Serialize)]
pub struct AdminCommunityInfoResponse {
    community: Community,
    users: Vec<AdminCommunityInfoResponseUser>,
    managers: Vec<AdminCommunityInfoResponseUser>,
    admins: Vec<AdminCommunityInfoResponseUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditCommunityRequest {
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeMembersCommunityRequest {
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CommunityCreateRequest {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be at least 2 characters and at most 100 characters"
    ))]
    pub name: String,
    #[validate(length(min = 2, message = "Description must be at least 2 characters"))]
    pub description: String,
    #[validate(url(message = "Image URL must be valid"))]
    #[serde(deserialize_with = "empty_string_as_none")]
    pub image: Option<String>,
}

fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

async fn require_manage_permission_and_find_user(
    state: &AppState,
    user_id: Uuid,
    community_id: Uuid,
    user_email: &str,
) -> AppResult<User> {
    let user = state
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::UserNotFoundId(user_id))?;

    if !state.can_manage_community(&user, community_id).await? {
        return Err(AppError::Unauthorized);
    }

    let target_user = state
        .get_user_by_email(user_email)
        .await?
        .ok_or_else(|| AppError::UserNotFoundEmail(user_email.to_string()))?;

    Ok(target_user)
}

#[debug_handler]
pub async fn get_admin_manageable_communities(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<AdminListCommunityView>>> {
    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or_else(|| AppError::UserNotFoundId(session.user_id))?;

    let communities = state
        .list_admin_community_view(user.id, user.is_admin)
        .await?;

    Ok(Json(communities))
}

#[debug_handler]
pub async fn create_community(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<CommunityCreateRequest>,
) -> AppResult<impl IntoResponse> {
    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or(AppError::UserNotFoundId(session.user_id))?;

    if !user.is_admin {
        return Err(AppError::Unauthorized);
    }

    let community = state
        .create_community(
            &request.name,
            &request.description,
            request.image.as_deref(),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(community)))
}

#[debug_handler]
pub async fn get_admin_community_information(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<Json<AdminCommunityInfoResponse>> {
    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or(AppError::UserNotFoundId(session.user_id))?;

    if !state.can_manage_community(&user, id).await? {
        return Err(AppError::Unauthorized);
    }

    let community = state
        .get_community_by_id(id)
        .await?
        .ok_or(AppError::CommunityNotFound(id))?;

    let (users, managers, admins) = tokio::try_join!(
        state.get_users_from_community(id),
        state.get_managers_from_community(id),
        state.get_admins()
    )?;

    let response = AdminCommunityInfoResponse {
        community,
        users: users.into_iter().map(Into::into).collect(),
        managers: managers.into_iter().map(Into::into).collect(),
        admins: admins.into_iter().map(Into::into).collect(),
    };

    Ok(Json(response))
}

#[debug_handler]
pub async fn edit_community_information(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<EditCommunityRequest>,
) -> AppResult<StatusCode> {
    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or(AppError::UserNotFoundId(session.user_id))?;

    if !state.can_manage_community(&user, id).await? {
        return Err(AppError::Unauthorized);
    }

    let community = state
        .get_community_by_id(id)
        .await?
        .ok_or(AppError::CommunityNotFound(id))?;

    state
        .update_community(
            community.id,
            request.name,
            request.description,
            request.image,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn add_user_to_community(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ChangeMembersCommunityRequest>,
) -> AppResult<StatusCode> {
    let target_user =
        require_manage_permission_and_find_user(&state, session.user_id, id, &request.user_email)
            .await?;
    state.add_user_to_community(id, target_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn remove_user_from_community(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ChangeMembersCommunityRequest>,
) -> AppResult<StatusCode> {
    let target_user =
        require_manage_permission_and_find_user(&state, session.user_id, id, &request.user_email)
            .await?;
    state.remove_user_from_community(id, target_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn add_manager_to_community(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ChangeMembersCommunityRequest>,
) -> AppResult<StatusCode> {
    let target_user =
        require_manage_permission_and_find_user(&state, session.user_id, id, &request.user_email)
            .await?;
    state.add_manager_to_community(id, target_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn remove_manager_from_community(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<ChangeMembersCommunityRequest>,
) -> AppResult<StatusCode> {
    let target_user =
        require_manage_permission_and_find_user(&state, session.user_id, id, &request.user_email)
            .await?;
    state
        .remove_manager_from_community(id, target_user.id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
