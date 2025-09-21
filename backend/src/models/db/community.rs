use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::models::db::participant::ParticipantRole;

#[derive(Debug, Deserialize, Serialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub image: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantCommunity {
    pub participant: Uuid,
    pub community: String,
    pub role: ParticipantRole,
}
