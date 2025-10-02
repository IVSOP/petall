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
#[serde(rename_all = "camelCase")]
struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    name: String,
    #[validate(email(message = "Invalid email address"))]
    email: String,
    #[validate(length(
        min = 8,
        max = 50,
        message = "Password must be between 8 and 50 characters"
    ))]
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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

    if state.get_user_by_email(&request.email).await?.is_some() {
        // email already exists
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = state
        .register_user(&request.email, &request.name, &request.password)
        .await?;

    let token_id = Uuid::new_v4();
    let (refresh_token, refresh_expiration) =
        jwt::create_refresh_token(&state.jwt_config, token_id, user.id)?;
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, user.id)?;

    state
        .store_token(token_id, user.id.clone(), refresh_expiration)
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
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[debug_handler]
async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let user = state
        .get_user_by_email(&request.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    if !password::verify_password(&request.password, &user.password)? {
        return Err(AppError::InvalidCredentials);
    }

    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, user.id)?;
    let (refresh_token, refresh_expiration) =
        jwt::create_refresh_token(&state.jwt_config, token_id, user.id)?;

    state
        .store_token(token_id, user.id.clone(), refresh_expiration)
        .await?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefreshTokenResponse {
    access_token: String,
}

#[debug_handler]
async fn refresh_handler(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> AppResult<impl IntoResponse> {
    let refresh_token = jwt::decode_refresh_token(&state.jwt_config, &request.refresh_token)
        .map_err(|_| AppError::InvalidToken)?;

    if !state.is_token_valid(refresh_token.token_id).await? {
        return Err(AppError::InvalidToken);
    }

    let access_token =
        jwt::create_access_token(&state.jwt_config, refresh_token.token_id, refresh_token.sub)?;

    Ok(Json(RefreshTokenResponse { access_token }))
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeResponse {
    pub message: String,
}

#[debug_handler]
async fn revoke_handler(
    ExtractAccessToken(access_token): ExtractAccessToken,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    if !state.delete_token(access_token.token_id).await? {
        return Err(AppError::InvalidToken);
    }

    Ok(Json(RevokeResponse {
        message: "Token revoked".to_string(),
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    let user = state
        .get_user_by_id(&access_token.sub)
        .await?
        .ok_or(AppError::InvalidToken)?;

    if !password::verify_password(&request.old_password, &user.password)? {
        return Err(AppError::InvalidCredentials);
    }

    state
        .update_user_password(&user.id, &request.new_password)
        .await?;

    let token_id = Uuid::new_v4();
    let access_token = jwt::create_access_token(&state.jwt_config, token_id, user.id)?;
    let (refresh_token, refresh_expiration) =
        jwt::create_refresh_token(&state.jwt_config, token_id, user.id)?;

    state.delete_all_tokens(user.id).await?;

    state
        .store_token(token_id, user.id.clone(), refresh_expiration)
        .await?;

    Ok(Json(ChangePasswordResponse {
        access_token,
        refresh_token,
        message: "Password changed successfully".to_string(),
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    let user = state
        .get_user_by_id(&access_token.sub)
        .await?
        .ok_or(AppError::InvalidToken)?;

    Ok(Json(MeResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    }))
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use rsa::pkcs8::EncodePrivateKey;
    use rsa::pkcs8::EncodePublicKey;

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
    };

    fn generate_jwt_config() -> JwtConfig {
        let bits = 2048;
        let private_key = rsa::RsaPrivateKey::new(&mut rand::thread_rng(), bits)
            .expect("failed to generate a key");
        let public_key = rsa::RsaPublicKey::from(&private_key);

        let private_key_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .expect("failed to serialize private key");
        let public_key_pem = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .expect("failed to serialize public key");

        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .expect("failed to create encoding key");
        let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
            .expect("failed to create decoding key");

        JwtConfig {
            access_token_max_age: Duration::from_secs(3600), // 1 hour
            refresh_token_max_age: Duration::from_secs(86400), // 24 hours
            access_token_public_key: decoding_key.clone(),
            access_token_private_key: encoding_key.clone(),
            refresh_token_public_key: decoding_key.clone(),
            refresh_token_private_key: encoding_key.clone(),
        }
    }

    fn server(pg_pool: PgPool) -> TestServer {
        let jwt_config = Arc::new(generate_jwt_config());
        let state = crate::AppState {
            pg_pool,
            jwt_config,
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

        let empty_revoke_response = server.post("/revoke").await;
        empty_revoke_response.assert_status(StatusCode::UNAUTHORIZED);

        let access_token = register_response.access_token;

        let response = server
            .post("/revoke")
            .add_header("Authorization", &access_token)
            .await;
        response.assert_status(StatusCode::OK);

        let response = server
            .post("/revoke")
            .add_header("Authorization", &access_token)
            .await;
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
            .add_header("Authorization", &access_token)
            .json(&change_password_request)
            .await;
        response.assert_status(StatusCode::OK);

        let wrong_old_password_request = ChangePasswordRequest {
            old_password: "wrong_old_password".to_string(),
            new_password: "new_password2".to_string(),
        };

        let response = server
            .post("/changepassword")
            .add_header("Authorization", &access_token)
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
            .add_header("Authorization", &access_token)
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
