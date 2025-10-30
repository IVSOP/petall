use crate::AppState;
use crate::auth::extractor::ExtractSession;
use crate::error::{AppError, AppResult, ValidatedJson};
use crate::models::Community;
use axum::extract::Path;
use axum::{Json, debug_handler, extract::State, response::IntoResponse};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommunityResponse {
    #[serde(flatten)]
    pub community: Community,
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

// #[cfg(test)]
// mod tests {
//     use axum::http::StatusCode;
//     use sqlx::PgPool;
//     use tracing_test::traced_test;

//     use crate::{
//         auth::router::{RegisterRequest, RegisterResponse},
//         models::Community,
//         router::test_utils::test_server,
//     };

//     use super::{CommunityCreateRequest, UserCommunityResponse};

//     #[traced_test]
//     #[sqlx::test]
//     fn integration_test_community(pool: PgPool) {
//         let server = test_server(pool);

//         // Register a user
//         let request_body = RegisterRequest {
//             name: "test_user".to_string(),
//             email: "test@example.com".to_string(),
//             password: "password".to_string(),
//             is_admin: false,
//         };

//         let register_response = server.post("/auth/register").json(&request_body).await;
//         register_response.assert_status(StatusCode::OK);

//         let register_response = register_response.json::<RegisterResponse>();

//         // List communities if it is zero
//         let list_communities_response = server
//             .get("/community")
//             .add_header("Authorization", register_response.session_id.to_string())
//             .await
//             .json::<Vec<UserCommunityResponse>>();

//         assert_eq!(list_communities_response.len(), 0);

//         // Create a community
//         let name = "Test Community";
//         let description = "This is a test community";

//         let create_community_request = CommunityCreateRequest {
//             name: name.to_string(),
//             description: description.to_string(),
//             image: None,
//         };

//         let create_community_response = server
//             .post("/community")
//             .json(&create_community_request)
//             .add_header("Authorization", register_response.session_id.to_string())
//             .await;
//         create_community_response.assert_status(StatusCode::CREATED);
//         let community = create_community_response.json::<Community>();

//         assert_eq!(community.name, name);
//         assert_eq!(community.description, description);

//         // Get the community with request for individual communities
//         let get_community_response = server
//             .get(&format!("/community/{}", community.id))
//             .add_header("Authorization", register_response.session_id.to_string())
//             .await;
//         get_community_response.assert_status(StatusCode::OK);
//         let get_community = get_community_response.json::<UserCommunityResponse>();

//         let community_id = community.id;

//         assert_eq!(get_community.community.id, community_id);
//         assert_eq!(get_community.community.name, name);
//         assert_eq!(get_community.community.description, description);
//         assert_eq!(get_community.community.image, None);

//         // Get the community with request for all communities
//         let list_communities_response = server
//             .get("/community")
//             .add_header("Authorization", register_response.session_id.to_string())
//             .await;
//         list_communities_response.assert_status(StatusCode::OK);
//         let list_communities = list_communities_response.json::<Vec<Community>>();

//         assert_eq!(list_communities.len(), 1);
//         assert_eq!(list_communities[0].id, community_id);
//         assert_eq!(list_communities[0].name, name);
//         assert_eq!(list_communities[0].description, description);
//         assert_eq!(list_communities[0].image, None);
//     }
// }
