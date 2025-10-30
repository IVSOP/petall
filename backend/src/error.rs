use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;
use validator::Validate;

use crate::auth;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

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
    #[error("user not found using email: {0}")]
    UserNotFoundEmail(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid session")]
    InvalidSession,
    #[error("email already in use: {0}")]
    EmailAlreadyInUse(String),
    #[error("argon2 error: {0}")]
    Argon2Error(#[from] auth::password::Argon2Error),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error("OAuth error: {0}")]
    OAuthError(String),
    #[error("email not verified by provider")]
    EmailNotVerified,
    #[error("user already added to community: {0}")]
    UserAlreadyAddedToCommunity(Uuid),
    #[error("manager already added to community: {0}")]
    ManagerAlreadyAddedToCommunity(Uuid),
    #[error("user not in community: {0}")]
    UserNotInCommunity(Uuid),
    #[error("manager not in community: {0}")]
    ManagerNotInCommunity(Uuid),
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
            AppError::OAuthError(msg) => (
                axum::http::StatusCode::BAD_REQUEST,
                format!("OAuth error: {}", msg),
            ),
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
                format!("user not found with id: {id}"),
            ),
            AppError::UserNotFoundEmail(email) => (
                StatusCode::NOT_FOUND,
                format!("user not found with email: {email}"),
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
            AppError::AxumJsonRejection(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::EmailNotVerified => (
                StatusCode::FORBIDDEN,
                "Email not verified by provider".to_string(),
            ),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::UserAlreadyAddedToCommunity(id) => (
                StatusCode::CONFLICT,
                format!("User already added to community: {}", id),
            ),
            AppError::ManagerAlreadyAddedToCommunity(id) => (
                StatusCode::CONFLICT,
                format!("Manager already added to community: {}", id),
            ),
            AppError::UserNotInCommunity(id) => (
                StatusCode::FORBIDDEN,
                format!("User not in community: {}", id),
            ),
            AppError::ManagerNotInCommunity(id) => (
                StatusCode::FORBIDDEN,
                format!("Manager not in community: {}", id),
            ),
        };

        let body = ErrorBody { error };

        (status, Json(body)).into_response()
    }
}
