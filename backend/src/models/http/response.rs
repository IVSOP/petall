use crate::models::db::community::Community;
use crate::models::db::participant::ParticipantRole;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ParticipantCommunityResponse {
    pub community: Community,
    pub role: ParticipantRole,
}
