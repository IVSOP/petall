use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

use crate::auth;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("community not found: {0}")]
    CommunityNotFound(Uuid),
    #[error("community name already in use: {0}")]
    CommunityNameAlreadyInUse(String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::error::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("user not found using ID: {0}")]
    UserNotFoundId(Uuid),
    #[error("user-community already in use: user {0}, community {1}")]
    UserCommunityAlreadyInUse(Uuid, Uuid),
    #[error("not found user {0} in community {1}")]
    UserCommunityNotFound(Uuid, Uuid),
    #[error("invalid session")]
    InvalidSession,
    #[error("email already in use: {0}")]
    EmailAlreadyInUse(String),
    #[error("argon2 error: {0}")]
    Argon2Error(#[from] auth::password::Argon2Error),
    #[error("invalid credentials")]
    InvalidCredentials,
    // #[error("user not found using email: {0}")]
    // UserNotFoundEmail(String),
    // #[error("manager not found using ID: {0}")]
    // ManagerNotFoundId(Uuid),
    // #[error("invalid password")]
    // InvalidPassword,
    // #[error("invalid token")]
    // InvalidToken,
    // #[error("unauthorized")]
    // Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
        }

        let internal_server_error = (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal server error"),
        );

        let (status, error) = match &self {
            AppError::CommunityNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Community not found: {}", id),
            ),
            AppError::CommunityNameAlreadyInUse(name) => (
                StatusCode::CONFLICT,
                format!("Community name already in use: {}", name),
            ),
            AppError::SqlxError(err) => {
                error!("Database error occurred: {err:?}");
                internal_server_error
            }
            AppError::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                itertools::Itertools::intersperse(
                    err.field_errors()
                        .into_values()
                        .flatten()
                        .filter_map(|v| v.message.clone())
                        .map(|a| a.to_string()),
                    ", ".to_string(),
                )
                .collect(),
            ),
            AppError::UserNotFoundId(id) => (
                StatusCode::NOT_FOUND,
                format!("user not found with id: {}", id),
            ),
            AppError::UserCommunityAlreadyInUse(user, community) => (
                StatusCode::CONFLICT,
                format!(
                    "user-community already in use: user {}, community {}",
                    user, community
                ),
            ),
            AppError::UserCommunityNotFound(user, community) => (
                StatusCode::NOT_FOUND,
                format!("not found user {} in community {}", user, community),
            ),
            AppError::InvalidSession => (StatusCode::UNAUTHORIZED, "Invalid session".to_string()),
            AppError::EmailAlreadyInUse(email) => (
                StatusCode::CONFLICT,
                format!("Email already in use: {}", email),
            ),
            AppError::Argon2Error(argon2_error) => {
                error!("Argon2 error: {}", argon2_error);
                internal_server_error
            }
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
        };

        let body = ErrorBody { error };

        (status, Json(body)).into_response()
    }
}
