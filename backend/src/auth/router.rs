use crate::{
    AppState,
    auth::{extractor::ExtractAccessToken, jwt, password},
    error::{AppError, AppResult},
};
use axum::{Json, Router, debug_handler, extract::State, response::IntoResponse, routing::post};
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/revoke", post(revoke_handler))
        .route("/changepassword", post(change_password_handler))
        .route("/me", post(me_handler))
}

#[derive(serde::Deserialize, serde::Serialize, Validate)]
struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 50))]
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RevokeResponse {
    pub message: String,
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

#[derive(serde::Serialize, serde::Deserialize)]
struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
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

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use sqlx::PgPool;
    use tower_http::trace::TraceLayer;
    use tracing_test::traced_test;

    use crate::auth::{
        jwt::JwtConfig,
        router::{
            ChangePasswordRequest, LoginRequest, MeResponse, RefreshTokenRequest, RegisterRequest,
            RegisterResponse,
        },
        token_store,
    };

    fn generate_jwt_config() -> JwtConfig {
        const TEST_SECRET: &[u8] = b"test_secret";

        JwtConfig {
            access_token_max_age: Duration::from_secs(3600), // 1 hour
            refresh_token_max_age: Duration::from_secs(86400), // 24 hours
            access_token_public_key: DecodingKey::from_secret(TEST_SECRET),
            access_token_private_key: EncodingKey::from_secret(TEST_SECRET),
            refresh_token_public_key: DecodingKey::from_secret(TEST_SECRET),
            refresh_token_private_key: EncodingKey::from_secret(TEST_SECRET),
        }
    }

    fn server(pg_pool: PgPool) -> TestServer {
        let jwt_config = Arc::new(generate_jwt_config());
        let token_store = Arc::new(token_store::Store);
        let state = crate::AppState {
            pg_pool,
            jwt_config,
            token_store,
        };
        let router = crate::auth::router::router()
            .with_state(state)
            .layer(TraceLayer::new_for_http());

        TestServer::new(router).unwrap()
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_register(pool: PgPool) {
        let server = server(pool);

        let request_body = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/register").json(&request_body).await;
        response.assert_status(StatusCode::OK);

        let response = server.post("/register").json(&request_body).await;
        response.assert_status(StatusCode::CONFLICT);
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_login(pool: PgPool) {
        let server = server(pool);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/register").json(&register_request).await;
        response.assert_status(StatusCode::OK);

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/login").json(&login_request).await;
        response.assert_status(StatusCode::OK);

        let wrong_password_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let response = server.post("/login").json(&wrong_password_request).await;
        response.assert_status(StatusCode::UNAUTHORIZED);

        let nonexistent_email_request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/login").json(&nonexistent_email_request).await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_refresh(pool: PgPool) {
        let server = server(pool);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/register").json(&register_request).await;
        response.assert_status(StatusCode::OK);
        let register_response: RegisterResponse = response.json();

        let refresh_request = RefreshTokenRequest {
            refresh_token: register_response.refresh_token,
        };

        let response = server.post("/refresh").json(&refresh_request).await;
        response.assert_status(StatusCode::OK);

        let invalid_refresh_request = RefreshTokenRequest {
            refresh_token: "invalid_token".to_string(),
        };

        let response = server.post("/refresh").json(&invalid_refresh_request).await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_revoke(pool: PgPool) {
        let server = server(pool);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/register").json(&register_request).await;
        response.assert_status(StatusCode::OK);
        let register_response: RegisterResponse = response.json();

        let access_token = register_response.access_token;
        let response = server
            .post("/revoke")
            .add_header("Authorization", format!("Bearer {}", access_token))
            .await;
        response.assert_status(StatusCode::OK);

        let response = server.post("/revoke").await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_change_password(pool: PgPool) {
        let server = server(pool);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "old_password".to_string(),
        };

        let response = server.post("/register").json(&register_request).await;
        response.assert_status(StatusCode::OK);
        let register_response: RegisterResponse = response.json();

        let access_token = register_response.access_token;

        let change_password_request = ChangePasswordRequest {
            old_password: "old_password".to_string(),
            new_password: "new_password".to_string(),
        };

        let response = server
            .post("/changepassword")
            .add_header("Authorization", format!("Bearer {}", access_token))
            .json(&change_password_request)
            .await;
        response.assert_status(StatusCode::OK);

        let wrong_old_password_request = ChangePasswordRequest {
            old_password: "wrong_old_password".to_string(),
            new_password: "new_password2".to_string(),
        };

        let response = server
            .post("/changepassword")
            .add_header("Authorization", format!("Bearer {}", access_token))
            .json(&wrong_old_password_request)
            .await;
        response.assert_status(StatusCode::UNAUTHORIZED);

        let response = server
            .post("/changepassword")
            .json(&change_password_request)
            .await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[traced_test]
    #[sqlx::test]
    async fn test_me(pool: PgPool) {
        let server = server(pool);

        let register_request = RegisterRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
        };

        let response = server.post("/register").json(&register_request).await;
        response.assert_status(StatusCode::OK);
        let register_response: RegisterResponse = response.json();

        let access_token = register_response.access_token;

        let response = server
            .post("/me")
            .add_header("Authorization", format!("Bearer {}", access_token))
            .await;
        response.assert_status(StatusCode::OK);

        let me_response: MeResponse = response.json();

        assert_eq!(register_response.uuid, me_response.id);
        assert_eq!(me_response.email, "test@example.com");
        assert_eq!(me_response.name, "Test User");

        let response = server.post("/me").await;
        response.assert_status(StatusCode::UNAUTHORIZED);
    }
}
