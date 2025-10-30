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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use tracing_test::traced_test;

    use crate::{
        auth::router::{RegisterRequest, RegisterResponse},
        controller::admin::AdminListCommunityView,
        models::Community,
        router::test_utils::test_server,
    };

    use super::{
        AdminCommunityInfoResponse, ChangeMembersCommunityRequest, CommunityCreateRequest,
        EditCommunityRequest,
    };

    #[traced_test]
    #[sqlx::test]
    fn integration_test_admin_community_crud(pool: PgPool) {
        let server = test_server(pool);

        // Register an admin user
        let request_body = RegisterRequest {
            name: "admin_user".to_string(),
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
        };

        let register_response = server.post("/auth/register").json(&request_body).await;
        register_response.assert_status(StatusCode::OK);

        let register_response = register_response.json::<RegisterResponse>();

        // List manageable communities (should be empty initially)
        let list_communities_response = server
            .get("/admin/community")
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        list_communities_response.assert_status(StatusCode::OK);
        let list_communities = list_communities_response.json::<Vec<AdminListCommunityView>>();

        assert_eq!(list_communities.len(), 0);

        // Create a community
        let name = "Test Community";
        let description = "This is a test community";

        let create_community_request = CommunityCreateRequest {
            name: name.to_string(),
            description: description.to_string(),
            image: None,
        };

        let create_community_response = server
            .post("/admin/community")
            .json(&create_community_request)
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        create_community_response.assert_status(StatusCode::CREATED);
        let community = create_community_response.json::<Community>();

        assert_eq!(community.name, name);
        assert_eq!(community.description, description);
        assert_eq!(community.image, None);

        // List manageable communities (should have 1 community)
        let list_communities_response = server
            .get("/admin/community")
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        list_communities_response.assert_status(StatusCode::OK);
        let list_communities = list_communities_response.json::<Vec<AdminListCommunityView>>();

        assert_eq!(list_communities.len(), 1);
        assert_eq!(list_communities[0].community.id, community.id);
        assert_eq!(list_communities[0].community.name, name);
        assert_eq!(list_communities[0].community.description, description);

        // Get admin community information
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.community.id, community.id);
        assert_eq!(info.community.name, "Test Community");
        assert_eq!(info.community.description, "This is a test community");
        assert_eq!(info.users.len(), 0);
        assert_eq!(info.managers.len(), 0);
        assert_eq!(info.admins.len(), 1); // The admin we created

        // Edit community information
        let edit_request = EditCommunityRequest {
            name: Some("Updated Community".to_string()),
            description: Some("Updated description".to_string()),
            image: Some("https://example.com/image.png".to_string()),
        };

        let edit_response = server
            .patch(&format!("/admin/community/{}", community.id))
            .json(&edit_request)
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        edit_response.assert_status(StatusCode::NO_CONTENT);

        // Verify the changes
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.community.name, "Updated Community");
        assert_eq!(info.community.description, "Updated description");
        assert_eq!(
            info.community.image,
            Some("https://example.com/image.png".to_string())
        );
    }

    #[traced_test]
    #[sqlx::test]
    fn integration_test_admin_manage_members(pool: PgPool) {
        let server = test_server(pool);

        // Register an admin user
        let admin_request = RegisterRequest {
            name: "admin_user".to_string(),
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
        };

        let admin_response = server.post("/auth/register").json(&admin_request).await;
        admin_response.assert_status(StatusCode::OK);
        let admin_response = admin_response.json::<RegisterResponse>();

        // Register a regular user
        let user_request = RegisterRequest {
            name: "regular_user".to_string(),
            email: "user@example.com".to_string(),
            password: "password".to_string(),
            is_admin: false,
        };

        let user_response = server.post("/auth/register").json(&user_request).await;
        user_response.assert_status(StatusCode::OK);

        // Register a manager user
        let manager_request = RegisterRequest {
            name: "manager_user".to_string(),
            email: "manager@example.com".to_string(),
            password: "password".to_string(),
            is_admin: false,
        };

        let manager_response = server.post("/auth/register").json(&manager_request).await;
        manager_response.assert_status(StatusCode::OK);

        // Create a community
        let create_community_request = CommunityCreateRequest {
            name: "Test Community".to_string(),
            description: "This is a test community".to_string(),
            image: None,
        };

        let create_community_response = server
            .post("/admin/community")
            .json(&create_community_request)
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        create_community_response.assert_status(StatusCode::CREATED);
        let community = create_community_response.json::<Community>();

        // Add user to community
        let add_user_request = ChangeMembersCommunityRequest {
            user_email: "user@example.com".to_string(),
        };

        let add_user_response = server
            .put(&format!("/admin/community/{}/user", community.id))
            .json(&add_user_request)
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        add_user_response.assert_status(StatusCode::NO_CONTENT);

        // Verify user was added
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.users.len(), 1);
        assert_eq!(info.users[0].email, "user@example.com");

        // Remove user from community
        let remove_user_request = ChangeMembersCommunityRequest {
            user_email: "user@example.com".to_string(),
        };

        let remove_user_response = server
            .delete(&format!("/admin/community/{}/user", community.id))
            .json(&remove_user_request)
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        remove_user_response.assert_status(StatusCode::NO_CONTENT);

        // Verify user was removed
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.users.len(), 0);

        // Add manager to community
        let add_manager_request = ChangeMembersCommunityRequest {
            user_email: "manager@example.com".to_string(),
        };

        let add_manager_response = server
            .put(&format!("/admin/community/{}/manager", community.id))
            .json(&add_manager_request)
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        add_manager_response.assert_status(StatusCode::NO_CONTENT);

        // Verify manager was added
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.managers.len(), 1);
        assert_eq!(info.managers[0].email, "manager@example.com");

        // Remove manager from community
        let remove_manager_request = ChangeMembersCommunityRequest {
            user_email: "manager@example.com".to_string(),
        };

        let remove_manager_response = server
            .delete(&format!("/admin/community/{}/manager", community.id))
            .json(&remove_manager_request)
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        remove_manager_response.assert_status(StatusCode::NO_CONTENT);

        // Verify manager was removed
        let get_info_response = server
            .get(&format!("/admin/community/{}", community.id))
            .add_header("Authorization", admin_response.session_id.to_string())
            .await;
        get_info_response.assert_status(StatusCode::OK);
        let info = get_info_response.json::<AdminCommunityInfoResponse>();

        assert_eq!(info.managers.len(), 0);
    }
}
