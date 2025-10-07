use crate::{
    AppState,
    auth::{Session, extractor::ExtractSession, password},
    error::{AppError, AppResult},
    models::Key,
};
use axum::{Json, Router, debug_handler, extract::State, response::IntoResponse, routing::post};
use uuid::Uuid;
use validator::Validate;
use crate::auth::oauth;
use oauth2::TokenResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/revoke", post(revoke_handler))
        .route("/changepassword", post(change_password_handler))
        .route("/me", post(me_handler))
        .route("/oauth/google", axum::routing::get(google_oauth_handler))
        .route("/callback", axum::routing::get(google_callback_handler))
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
    session_id: Uuid,
}

#[debug_handler]
async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    request.validate()?;

    // email already exists
    if state.get_user_by_email(&request.email).await?.is_some() {
        return Err(AppError::EmailAlreadyInUse(request.email));
    }

    let user = state
        .register_user(&request.email, &request.name, &request.password)
        .await?;

    let session = Session::new_random_from(user.id);
    state.store_session(&session).await?;

    Ok(Json(RegisterResponse {
        uuid: user.id,
        name: user.name,
        email: user.email,
        session_id: session.id,
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
    uuid: Uuid,
    name: String,
    email: String,
    session_id: Uuid,
}


#[debug_handler]
async fn google_oauth_handler(
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let (auth_url, _csrf_token) = state.google_oauth.get_authorization_url();
    
    //TODO: save CRSF TOKEN in user session and verify it in callbacks
    
    Ok(axum::response::Redirect::to(auth_url.as_str()))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct OAuthCallbackQuery {
    code: String,
    state: String,  
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OAuthCallbackResponse {
    uuid: Uuid,
    name: String,
    email: String,
    session_id: Uuid,
    is_new_user: bool,
}

#[debug_handler]
async fn google_callback_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<OAuthCallbackQuery>,
) -> AppResult<impl IntoResponse> {
    // TODO: verify CRSF here
    let token_response = state
        .google_oauth
        .exchange_code(query.code)
        .await
        .map_err(|e| AppError::OAuthError(e))?;

    let access_token = token_response.access_token().secret();

    let google_user = oauth::get_google_user_info(access_token)
        .await
        .map_err(|e| AppError::OAuthError(e))?;

    if !google_user.email_verified {
        return Err(AppError::OAuthError(
            "Email not verified by Google".to_string()
        ));
    }

    let key_id = Key::oauth_key_id("google", &google_user.sub);

    let mut is_new_user = false;
    let user = if let Some(key) = state.get_key(&key_id).await? {
        // user already exists
        state
            .get_user_by_id(key.user_id)
            .await?
            .ok_or(AppError::InvalidSession)?
    } else {
        if let Some(existing_user) = state.get_user_by_email(&google_user.email).await? {
            //email already exists
            state
                .create_key(&key_id, existing_user.id, None)
                .await?;
            existing_user
        } else {
            is_new_user = true;
            let new_user = sqlx::query_as!(
                crate::models::User,
                r#"
                INSERT INTO "user" (email, name)
                VALUES ($1, $2)
                RETURNING *
                "#,
                google_user.email,
                google_user.name
            )
            .fetch_one(&state.pg_pool)
            .await?;

            state
                .create_key(&key_id, new_user.id, None)
                .await?;

            new_user
        }
    };

    let session = Session::new_random_from(user.id);
    state.store_session(&session).await?;

    Ok(Json(OAuthCallbackResponse {
        uuid: user.id,
        name: user.name,
        email: user.email,
        session_id: session.id,
        is_new_user,
    }))
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

    // search key 
    let key_id = Key::email_key_id(&request.email);
    let key = state
        .get_key(&key_id)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // does key have password? 
    let hashed_password = key
        .hashed_password
        .ok_or(AppError::InvalidCredentials)?;

    if !password::verify_password(&request.password, &hashed_password)? {
        return Err(AppError::InvalidCredentials);
    }

    let session = Session::new_random_from(user.id);
    state.store_session(&session).await?;

    Ok(Json(LoginResponse {
        uuid: user.id,
        name: user.name,
        email: user.email,
        session_id: session.id,
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeResponse {
    pub message: String,
}

#[debug_handler]
async fn revoke_handler(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    if !state.delete_session(session.id).await? {
        return Err(AppError::InvalidSession);
    }

    Ok(Json(RevokeResponse {
        message: "Session revoked".to_string(),
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
    session_id: Uuid,
}

#[debug_handler]
async fn change_password_handler(
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
    request: Json<ChangePasswordRequest>,
) -> AppResult<impl IntoResponse> {

    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or(AppError::InvalidSession)?;


    let key_id = Key::email_key_id(&user.email);
    let key = state
        .get_key(&key_id)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let hashed_password = key
        .hashed_password
        .ok_or(AppError::InvalidCredentials)?;

    if !password::verify_password(&request.old_password, &hashed_password)? {
        return Err(AppError::InvalidCredentials);
    }

    state
        .update_user_password(&user.id, &user.email, &request.new_password)
        .await?;

    state.delete_all_sessions(user.id).await?;

    let session = Session::new_random_from(user.id);
    state.store_session(&session).await?;

    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
        session_id: session.id,
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
    ExtractSession(session): ExtractSession,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = state
        .get_user_by_id(session.user_id)
        .await?
        .ok_or(AppError::InvalidSession)?;

    Ok(Json(MeResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    }))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use sqlx::PgPool;
    use tower_http::trace::TraceLayer;
    use tracing_test::traced_test;

    use crate::auth::router::{
        ChangePasswordRequest, LoginRequest, MeResponse, RegisterRequest, RegisterResponse,
    };

    fn server(pg_pool: PgPool) -> TestServer {
        let state = crate::AppState { pg_pool };
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

        let session_id = register_response.session_id;

        let response = server
            .post("/revoke")
            .add_header("Authorization", session_id.to_string())
            .await;
        response.assert_status(StatusCode::OK);

        let response = server
            .post("/revoke")
            .add_header("Authorization", session_id.to_string())
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

        let session_id = register_response.session_id;

        let change_password_request = ChangePasswordRequest {
            old_password: "old_password".to_string(),
            new_password: "new_password".to_string(),
        };

        let response = server
            .post("/changepassword")
            .add_header("Authorization", session_id.to_string())
            .json(&change_password_request)
            .await;
        response.assert_status(StatusCode::OK);

        let wrong_old_password_request = ChangePasswordRequest {
            old_password: "wrong_old_password".to_string(),
            new_password: "new_password2".to_string(),
        };

        let response = server
            .post("/changepassword")
            .add_header("Authorization", session_id.to_string())
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

        let session_id = register_response.session_id;

        let response = server
            .post("/me")
            .add_header("Authorization", session_id.to_string())
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