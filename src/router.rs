use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use crate::error::AppResult;

use super::AppState;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/example/get", get(list_communities))
        .route("/example/post", post(register_cominunity))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}


#[derive(serde::Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub entity: String,
}

#[derive(serde::Serialize)]
struct CommunityResponse {
    uuid: Uuid,
    entity: String,
}


#[debug_handler]
async fn register_cominunity(
    State(state): State<AppState>,
    Json(request): Json<CommunityRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;

    if user::get_user_from_email(request.email.clone(), &state)
        .await?
        .is_some()
    {
        // email already exists
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = user::register_user(request, &state).await?;

    let token_id = Uuid::new_v4();
    let refresh_token = jwt::create_refresh_token(&state, token_id, &user)?;
    let access_token = jwt::create_access_token(&state, token_id, &user)?;

    redis::store_token(token_id, user.uuid, &state).await?;

    let mut headers = HeaderMap::new();
    append_access_token_cookie(&mut headers, &access_token, &state);
    append_refresh_token_cookie(&mut headers, &refresh_token, &state);

    Ok((
        headers,
        Json(RegisterResponse {
            uuid: user.uuid,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            access_token,
            refresh_token,
        }),
    ))
}