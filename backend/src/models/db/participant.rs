use uuid::Uuid;
use serde::{Deserialize, Serialize};

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
    pub supplier: Uuid,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ParticipantAlias {
    pub participant: Uuid,
    pub alias: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ParticipantCommunity {
    pub participant: Uuid,
    pub community: Uuid,
    pub role: ParticipantRole,
}
