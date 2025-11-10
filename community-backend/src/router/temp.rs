use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use common::EnergyRecord;
use uuid::Uuid;

use crate::{
    AppState,
    error::{AppError, AppResult},
};

/// Temporary endpoint for the Proof of Concept zero-knowledge service.
///
/// Currently returns the original energy record without authentication.
/// This bypasses the planned architecture where the ZK service would maintain its own database
/// copy and generate proofs from snapshots. For now, the ZK service will just grab the origianl record.
/// This endpoint will be removed once the full zero-knowledge service integration is complete.
#[debug_handler]
pub async fn get_energy_record_unauthenticated(
    State(state): State<AppState>,
    Path(energy_record_id): Path<Uuid>,
) -> AppResult<Json<EnergyRecord>> {
    let Some(energy_record) = state.get_energy_record(energy_record_id).await? else {
        return Err(AppError::EnergyRecordNotFound(energy_record_id));
    };
    Ok(Json(energy_record))
}
