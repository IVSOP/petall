use crate::AppState;
use crate::auth;
use crate::auth::extractor::ExtractSession;
use crate::error::{AppError, AppResult, ValidatedJson};
use crate::models::Community;
use crate::models::UserRole;
use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{
    Json, debug_handler,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/community", get(get_communities_from_user))
        .route("/community", post(create_community))
        .route("/community/{id}", get(get_community_by_id))
        .route("/community/{id}/energy", post(list_user_energy_records))
        .nest("/auth", auth::router::router().with_state(state.clone()))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommunityResponse {
    #[serde(flatten)]
    community: Community,
    role: UserRole,
}

#[debug_handler]
pub async fn get_communities_from_user(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let communities = state
        .get_communities_from_user(session.user_id)
        .await?
        .into_iter()
        .map(|(community, role)| UserCommunityResponse { community, role })
        .collect::<Vec<_>>();

    Ok(Json(communities))
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

#[debug_handler]
pub async fn create_community(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<CommunityCreateRequest>,
) -> AppResult<impl IntoResponse> {
    let community = state
        .create_community(
            session.user_id,
            &request.name,
            &request.description,
            request.image.as_deref(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(community)))
}

#[debug_handler]
pub async fn get_community_by_id(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let response = state.get_community_by_id(session.user_id, id).await?;

    let Some(response) = response else {
        return Err(AppError::CommunityNotFound(id));
    };

    let (community, role) = response;

    Ok(Json(UserCommunityResponse { community, role }))
}

#[derive(Debug, Deserialize, Default)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    #[default]
    Descending,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnergyFilter {
    #[validate(range(min = 1, max = 100))]
    pub page: u32,
    #[validate(range(min = 1, max = 100))]
    pub size: u32,
    pub order_dir: OrderDirection,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

#[debug_handler]
pub async fn list_user_energy_records(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    ValidatedJson(query): ValidatedJson<EnergyFilter>,
) -> AppResult<impl IntoResponse> {
    let energy = state
        .get_user_energy_records(
            session.user_id,
            id,
            query.page,
            query.size,
            query.order_dir,
            query.start,
            query.end,
        )
        .await?;
    Ok(Json(energy))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use sqlx::PgPool;
    use tracing_test::traced_test;

    use crate::{
        auth::router::{RegisterRequest, RegisterResponse},
        models::{Community, UserRole},
        router::{CommunityCreateRequest, UserCommunityResponse, router},
    };

    fn server(pg_pool: PgPool) -> TestServer {
        let state = crate::AppState { pg_pool };
        let router = router(state);

        TestServer::new(router).unwrap()
    }

    #[traced_test]
    #[sqlx::test]
    fn integration_test_community(pool: PgPool) {
        let server = server(pool);

        // Register a user
        let request_body = RegisterRequest {
            name: "test_user".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let register_response = server.post("/auth/register").json(&request_body).await;
        register_response.assert_status(StatusCode::OK);

        let register_response = register_response.json::<RegisterResponse>();

        // List communities if it is zero
        let list_communities_response = server
            .get("/community")
            .add_header("Authorization", register_response.session_id.to_string())
            .await
            .json::<Vec<UserCommunityResponse>>();

        assert_eq!(list_communities_response.len(), 0);

        // Create a community
        let name = "Test Community";
        let description = "This is a test community";

        let create_community_request = CommunityCreateRequest {
            name: name.to_string(),
            description: description.to_string(),
            image: None,
        };

        let create_community_response = server
            .post("/community")
            .json(&create_community_request)
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        create_community_response.assert_status(StatusCode::CREATED);
        let community = create_community_response.json::<Community>();

        assert_eq!(community.name, name);
        assert_eq!(community.description, description);

        // Get the community with request for individual communities
        let get_community_response = server
            .get(&format!("/community/{}", community.id))
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        get_community_response.assert_status(StatusCode::OK);
        let get_community = get_community_response.json::<UserCommunityResponse>();

        let community_id = community.id;

        assert_eq!(get_community.role, UserRole::Manager);
        assert_eq!(get_community.community.id, community_id);
        assert_eq!(get_community.community.name, name);
        assert_eq!(get_community.community.description, description);
        assert_eq!(get_community.community.image, None);

        // Get the community with request for all communities
        let list_communities_response = server
            .get("/community")
            .add_header("Authorization", register_response.session_id.to_string())
            .await;
        list_communities_response.assert_status(StatusCode::OK);
        let list_communities = list_communities_response.json::<Vec<Community>>();

        assert_eq!(list_communities.len(), 1);
        assert_eq!(list_communities[0].id, community_id);
        assert_eq!(list_communities[0].name, name);
        assert_eq!(list_communities[0].description, description);
        assert_eq!(list_communities[0].image, None);
    }
}
