use std::time::Duration;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    AppState,
    auth::extractor::ExtractSession,
    error::{AppError, AppResult},
};

pub struct ValidationSigner {
    private_key: EncodingKey,
    max_age: Duration,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationClaims {
    /// User ID
    uid: Uuid,
    /// Energy Record ID
    eri: Uuid,
    /// Expiration Timestamp
    exp: i64,
}

impl ValidationSigner {
    pub fn new(private_key: EncodingKey, max_age: Duration) -> Self {
        Self {
            private_key,
            max_age,
        }
    }

    #[cfg(test)]
    pub fn test_new() -> Self {
        Self {
            private_key: EncodingKey::from_secret(b"secret"),
            max_age: Duration::from_secs(60),
        }
    }

    pub fn create_validation_token(
        &self,
        user_id: Uuid,
        energy_record_id: Uuid,
    ) -> AppResult<String> {
        let expiration = (Utc::now() + self.max_age).timestamp();

        let claims = ValidationClaims {
            uid: user_id,
            eri: energy_record_id,
            exp: expiration,
        };

        let key = &self.private_key;
        let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key)?;
        Ok(token)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignEnergyRecordValidationResponse {
    pub signed_request: String,
}

#[debug_handler]
pub async fn sign_energy_record_validation_request(
    ExtractSession(session): ExtractSession,
    Path(energy_record_id): Path<Uuid>,
    State(state): State<AppState>,
) -> AppResult<Json<SignEnergyRecordValidationResponse>> {
    let user_id = session.user_id;
    let Some(energy_record) = state.get_energy_record(energy_record_id).await? else {
        return Err(AppError::EnergyRecordNotFound(energy_record_id));
    };

    if energy_record.user_id != user_id {
        return Err(AppError::EnergyRecordNotFound(energy_record_id));
    }

    let token = state
        .validation_signer
        .create_validation_token(user_id, energy_record_id)?;

    Ok(Json(SignEnergyRecordValidationResponse {
        signed_request: token,
    }))
}
