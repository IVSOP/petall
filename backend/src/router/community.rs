use crate::AppState;
use crate::auth::extractor::ExtractSession;
use crate::error::{AppResult, ValidatedJson};
use crate::models::Community;
use crate::models::UserRole;
use axum::extract::Path;
use axum::{
    Json, Router, debug_handler,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/community", get(get_communities_from_user))
        .route("/community", post(create_community))
        .route("/community/{id}", get(get_community_by_id))
        .route("/community/{id}/energy", post(list_user_energy_records))
        .route("/community/{id}/stats", post(get_stats))
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize, Validate)]
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
    Ok(Json(community))
}

#[debug_handler]
pub async fn get_community_by_id(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let community = state.get_community_by_id(id, session.user_id).await?;
    Ok(Json(community))
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StatsGranularity {
    All,
    Daily,
    Weekly,
    Monthly,
    Yearly
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
}

#[debug_handler]
pub async fn get_stats(
    ExtractSession(session): ExtractSession,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(query): Json<StatsFilter>,
) -> AppResult<impl IntoResponse> {
    let stats = state
        .get_energy_records_stats(
            session.user_id,
            id,
            &query
        )
        .await?;

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
