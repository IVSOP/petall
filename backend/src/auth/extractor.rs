use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    AppState,
    auth::jwt::{self, AccessTokenClaims},
    error::AppError,
};

pub struct ExtractAccessToken(pub AccessTokenClaims);

impl FromRequestParts<AppState> for ExtractAccessToken {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::InvalidToken)?;

        let access_token = header.to_str().map_err(|_| AppError::InvalidToken)?;

        let token = jwt::decode_access_token(&state.jwt_config, access_token)
            .map_err(|_| AppError::InvalidToken)?;
        Ok(ExtractAccessToken(token))
    }
}
