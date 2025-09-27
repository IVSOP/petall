use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

use crate::auth;

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
    #[error("participant not found using ID: {0}")]
    ParticipantNotFoundId(Uuid),
    #[error("participant-community already in use: participant {0}, community {1}")]
    ParticipantCommunityAlredyInUse(Uuid, Uuid),
    #[error("not found participant {0} in community {1}")]
    ParticipantCommunityNotFound(Uuid, Uuid),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("invalid jwt token")]
    InvalidToken,
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

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<serde_json::Value>,
        }

        let internal_server_error = (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal server error"),
            None,
        );

        let (status, error, details) = match &self {
            AppError::CommunityNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Community not found: {}", id),
                None,
            ),
            AppError::CommunityNameAlreadyInUse(name) => (
                StatusCode::CONFLICT,
                format!("Community name already in use: {}", name),
                None,
            ),
            AppError::SqlxError(err) => {
                error!("Database error occurred: {err:?}");
                internal_server_error
            }
            AppError::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                serde_json::to_value(err).ok(),
            ),
            AppError::ParticipantNotFoundId(id) => (
                StatusCode::NOT_FOUND,
                format!("Participant not found with id: {}", id),
                None,
            ),
            AppError::ParticipantCommunityAlredyInUse(participant, community) => (
                StatusCode::CONFLICT,
                format!(
                    "participant-community already in use: participant {}, community {}",
                    participant, community
                ),
                None,
            ),
            AppError::ParticipantCommunityNotFound(participant, community) => (
                StatusCode::NOT_FOUND,
                format!(
                    "not found participant {} in community {}",
                    participant, community
                ),
                None,
            ),
            AppError::JwtError(error) => {
                error!("JWT error: {:?}", error);
                internal_server_error
            }
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string(), None),
            AppError::EmailAlreadyInUse(email) => (
                StatusCode::BAD_REQUEST,
                format!("Email already in use: {}", email),
                None,
            ),
            AppError::Argon2Error(argon2_error) => {
                error!("Argon2 error: {}", argon2_error);
                internal_server_error
            }
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_string(),
                None,
            ),
        };

        let body = ErrorBody { error, details };

        (status, Json(body)).into_response()
    }
}
