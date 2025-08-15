use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("community not found: {0}")]
    CommunityNotFound(Uuid),
    #[error("community entity already in use: {0}")]
    CommunityEntityAlreadyInUse(String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::error::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("user not found using ID: {0}")]
    UserNotFoundId(Uuid),
    // #[error("user not found using email: {0}")]
    // UserNotFoundEmail(String),
    #[error("manager not found using ID: {0}")]
    ManagerNotFoundId(Uuid),
    // #[error("invalid password")]
    // InvalidPassword,
    // #[error("invalid token")]
    // InvalidToken,
    // #[error("unauthorized")]
    // Unauthorized,
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

        let (status, error, details) = match &self {
            AppError::CommunityNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Community not found: {}", id),
                None,
            ),
            AppError::CommunityEntityAlreadyInUse(entity) => (
                StatusCode::BAD_REQUEST,
                format!("Community entity already in use: {}", entity),
                None,
            ),
            AppError::SqlxError(err) => {
                error!(error = ?err, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Internal server error"),
                    None,
                )
            }
            AppError::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                serde_json::to_value(err).ok(),
            ),
            AppError::UserNotFoundId(id) => (
                StatusCode::NOT_FOUND,
                format!("User not found with id: {}", id),
                None,
            ),
            AppError::ManagerNotFoundId(id) => (
                StatusCode::NOT_FOUND,
                format!("Manager not found with id: {}", id),
                None,
            ),
        };

        let body = ErrorBody { error, details };

        (status, Json(body)).into_response()
    }
}
