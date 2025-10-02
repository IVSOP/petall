use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::{AppState, auth::Session, error::AppError};

pub struct ExtractSession(pub Session);

impl FromRequestParts<AppState> for ExtractSession {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::InvalidSession)?;

        let session = header.to_str().map_err(|_| AppError::InvalidSession)?;
        let session_id: Uuid = session.parse().map_err(|_| AppError::InvalidSession)?;

        let session = state
            .get_valid_session(session_id)
            .await?
            .ok_or(AppError::InvalidSession)?;

        Ok(ExtractSession(session))
    }
}
