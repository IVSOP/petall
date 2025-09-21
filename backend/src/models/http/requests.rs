use uuid::Uuid;
use chrono::NaiveDateTime;
use validator::Validate;
use serde::Deserialize;
use crate::models::db::participant::ParticipantRole;

#[derive(Debug, Deserialize, Validate)]
pub struct CommunityRegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ParticipantCommunityRegisterRequest {
    pub participant: Uuid,
    pub role: ParticipantRole,
}

#[derive(Debug, Deserialize, Default)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    #[default]
    Descending,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnergyQuery {
    #[validate(range(min = 1, max = 100))]
    pub page: u32,
    #[validate(range(min = 1, max = 100))]
    pub size: u32,
    pub order_dir: OrderDirection,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}
