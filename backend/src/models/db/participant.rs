use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "lowercase")]
pub enum ParticipantRole {
    User,
    Manager,
    UserManager,
}

#[derive(Debug, Serialize)]
pub struct Participant {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ParticipantCommunity {
    pub participant: Uuid,
    pub community: Uuid,
    pub role: ParticipantRole,
}
