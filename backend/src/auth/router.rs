use crate::{
    AppState,
    auth::{extractor::ExtractAccessToken, jwt, password},
    error::{AppError, AppResult},
};
use axum::{Json, Router, debug_handler, extract::State, response::IntoResponse, routing::post};
use uuid::Uuid;
use validator::Validate;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/revoke", post(revoke_handler))
        .route("/changepassword", post(change_password_handler))
        .route("/me", post(me_handler))
        .with_state(state)
}

#[derive(serde::Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 50))]
    pub password: String,
}

#[derive(serde::Serialize)]
struct RegisterResponse {
    uuid: Uuid,
    name: String,
    email: String,
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;

    if state
        .get_participant_by_email(&request.email)
        .await?
        .is_some()
    {
        // email already exists
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = state
        .register_participant(&request.email, &request.name, &request.password)
        .await?;

    let token_id = Uuid::new_v4();
    let refresh_token = jwt::create_refresh_token(&state.jwt_config, token_id, user.id)?;
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, user.id)?;

    state
        .token_store
        .store_token(token_id, user.id.clone())
        .await?;

    Ok(Json(RegisterResponse {
        uuid: user.id,
        name: user.name,
        email: user.email,
        access_token,
        refresh_token,
    }))
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let participant = state
        .get_participant_by_email(&request.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    if !password::verify_password(&request.password, &participant.password)? {
        return Err(AppError::InvalidCredentials);
    }

    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, participant.id)?;
    let refresh_token = jwt::create_refresh_token(&state.jwt_config, token_id, participant.id)?;

    state
        .token_store
        .store_token(token_id, participant.id.clone())
        .await?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(serde::Deserialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(serde::Serialize)]
struct RefreshTokenResponse {
    access_token: String,
}

#[debug_handler]
async fn refresh_handler(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> AppResult<impl IntoResponse> {
    let refresh_token = jwt::decode_refresh_token(&state.jwt_config, &request.refresh_token)?;

    if state
        .token_store
        .is_token_revoked(refresh_token.token_id)
        .await?
    {
        return Err(AppError::InvalidToken);
    }

    let access_token =
        jwt::create_access_token(&state.jwt_config, refresh_token.token_id, refresh_token.sub)?;

    Ok(Json(RefreshTokenResponse { access_token }))
}

#[derive(serde::Serialize)]
struct RevokeResponse {
    message: String,
}

#[debug_handler]
async fn revoke_handler(
    ExtractAccessToken(access_token): ExtractAccessToken,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    state
        .token_store
        .delete_token(access_token.token_id)
        .await?;

    Ok(Json(RevokeResponse {
        message: "Token revoked".to_string(),
    }))
}

#[derive(serde::Deserialize)]
struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

#[derive(serde::Serialize)]
struct ChangePasswordResponse {
    message: String,
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn change_password_handler(
    ExtractAccessToken(access_token): ExtractAccessToken,
    State(state): State<AppState>,
    request: Json<ChangePasswordRequest>,
) -> AppResult<impl IntoResponse> {
    let participant = state
        .get_participant_by_id(&access_token.sub)
        .await?
        .ok_or(AppError::InvalidToken)?;

    if !password::verify_password(&request.old_password, &participant.password)? {
        return Err(AppError::InvalidCredentials);
    }

    state
        .update_participant_password(&participant.id, &request.new_password)
        .await?;

    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, participant.id)?;
    let refresh_token = jwt::create_refresh_token(&state.jwt_config, token_id, participant.id)?;

    state.token_store.delete_all_tokens(participant.id).await?;

    state
        .token_store
        .store_token(token_id, participant.id.clone())
        .await?;

    Ok(Json(ChangePasswordResponse {
        access_token,
        refresh_token,
        message: "Password changed successfully".to_string(),
    }))
}

#[derive(serde::Serialize)]
struct MeResponse {
    id: Uuid,
    email: String,
    name: String,
}

#[debug_handler]
async fn me_handler(
    ExtractAccessToken(access_token): ExtractAccessToken,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let participant = state
        .get_participant_by_id(&access_token.sub)
        .await?
        .ok_or(AppError::InvalidToken)?;

    Ok(Json(MeResponse {
        id: participant.id,
        email: participant.email,
        name: participant.name,
    }))
}
