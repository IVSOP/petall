use serde::Serialize;
use crate::models::db::community::Community;
use crate::models::db::participant::ParticipantRole;

#[derive(Debug, Serialize)]
pub struct ParticipantCommunityResponse {
    pub community: Community,
    pub role: ParticipantRole,
}
