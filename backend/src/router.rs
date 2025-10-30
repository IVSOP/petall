use crate::AppState;
use crate::auth;
use crate::auth::extractor::ExtractSession;
use crate::controller::admin::AdminListCommunityView;
use crate::error::{AppError, AppResult, ValidatedJson};
use crate::models::Community;
use crate::models::User;
use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::delete;
use axum::routing::patch;
use axum::routing::put;
use axum::{
    Json, debug_handler,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/admin/community", get(get_admin_manageable_communities))
        .route(
            "/admin/community/{id}",
            get(get_admin_community_information),
        )
        .route(
            "/admin/community/{id}/edit",
            patch(edit_community_information),
        )
        .route(
            "/admin/community/{id}/manager",
            put(add_manager_to_community),
        )
        .route(
            "/admin/community/{id}/manager",
            delete(remove_manager_from_community),
        )
        .route("/admin/community/{id}/user", put(add_user_to_community))
        .route(
            "/admin/community/{id}/user",
            delete(remove_user_from_community),
        )
        .route("/community", get(get_communities_from_user))
        .route("/community", post(create_community))
        .route("/community/{id}", get(get_community_by_id))
        .route("/community/{id}/energy", post(list_user_energy_records))
        .route("/community/{id}/stats", post(get_stats))
        .nest("/auth", auth::router::router().with_state(state.clone()))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommunityResponse {
    #[serde(flatten)]
    community: Community,
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
        .map(|community| UserCommunityResponse { community })
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
    ExtractSession(_session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    // TODO: idk if we need authentication here but whatever
    let response = state.get_community_by_id(id).await?;

    let Some(response) = response else {
        return Err(AppError::CommunityNotFound(id));
    };

    Ok(Json(UserCommunityResponse {
        community: response,
    }))
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
    #[validate(range(min = 1))]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsGranularity {
    All,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsFilter {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub granularity: StatsGranularity,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EnergyStats {
    pub period_start: NaiveDateTime,

    #[serde(with = "bigdecimal::serde::json_num")]
    pub generated_sum: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumed_sum: BigDecimal,

    #[serde(with = "bigdecimal::serde::json_num")]
    pub generated_price: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumed_price: BigDecimal,
}

#[debug_handler]
pub async fn get_stats(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(query): Json<StatsFilter>,
) -> AppResult<impl IntoResponse> {
    let stats = state
        .get_energy_records_stats(session.user_id, id, &query)
        .await?;

    // debug porque o NaiveDateTime é uma merda
    // let stats_filter = StatsFilter {
    //     start: NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
    //         .expect("Failed to parse start date"),
    //     end: NaiveDateTime::parse_from_str("2025-12-31 23:59:59", "%Y-%m-%d %H:%M:%S")
    //         .expect("Failed to parse end date"),
    //     granularity: StatsGranularity::Monthly,
    // };

    // // Serialize to JSON
    // let json_output = serde_json::to_string_pretty(&stats_filter).unwrap();
    // error!("{}", json_output);
    // let stats: Vec<EnergyStats> = Vec::new();

    // info!("returning {} stats", stats.len());

    Ok(Json(stats))
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditCommunityRequest {
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct ChangeMembersCommunityRequest {
    pub user_email: String,
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
    use axum_test::TestServer;
    use sqlx::PgPool;
    use tracing_test::traced_test;

    use crate::{
        auth::router::{RegisterRequest, RegisterResponse},
        models::Community,
        router::{CommunityCreateRequest, UserCommunityResponse, router},
    };

    fn server(pg_pool: PgPool) -> TestServer {
        let google_oauth = crate::auth::oauth::GoogleOAuthClient::new(
            "test-client-id".to_string(),
            "test-client-secret".to_string(),
            "http://localhost:8080/auth/callback".to_string(),
        )
        .unwrap();

        let state = crate::AppState {
            pg_pool,
            google_oauth,
        };
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
            is_admin: false,
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
