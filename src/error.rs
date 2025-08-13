use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already in use: {0}")]
    EmailAlreadyInUse(String),
    #[error("email not found: {0}")]
    EmailNotFound(String),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlxError(#[from] sqlx::error::Error),
    #[error("invalid password")]
    InvalidPassword,
    #[error("invalid token")]
    InvalidToken,
    #[error("unauthorized")]
    Unauthorized,
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<serde_json::Value>,
        }

        let internal_server_error = || {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
                None,
            )
        };

        let (status, error, details) = match &self {
            AppError::EmailAlreadyInUse(email) => (
                StatusCode::BAD_REQUEST,
                format!("Email already in use: {}", email),
                None,
            ),
            AppError::EmailNotFound(email) => (
                StatusCode::NOT_FOUND,
                format!("Email not found: {}", email),
                None,
            ),
            AppError::InvalidPassword => (
                StatusCode::UNAUTHORIZED,
                "Invalid password".to_string(),
                None,
            ),
            AppError::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                serde_json::to_value(err).ok(),
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid refresh token".to_string(),
                None,
            ),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None),
            AppError::SqlxError(err) => {
                error!(error = ?err, "Database error occurred");
                internal_server_error()
            }
        };

        let body = ErrorBody { error, details };

        (status, Json(body)).into_response()
    }
}
