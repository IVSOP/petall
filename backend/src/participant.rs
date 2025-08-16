use crate::{AppState};
use serde::{Deserialize, Serialize};
use crate::router::EnergyTransferQuery;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

const TRANSFERS_PER_PAGE: u32 = 10;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "lowercase")]
pub enum ParticipantRole {
    Manager,
    Supplier,
    User,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct Participant {
    pub id: Uuid,
    pub role: ParticipantRole,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantCommunity {
    pub participant_id: Uuid,
    pub community_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyTransfer {
    pub id: Uuid,
    pub participant_from: Uuid,
    pub participant_to: Uuid,
    pub community: Uuid,
    pub energy_wh: BigDecimal,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl AppState {
    pub async fn get_participant_by_id(&self, id: &Uuid) -> sqlx::Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT id, role as "role: ParticipantRole", name, email FROM "participant"
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn _get_participant_by_email(&self, email: &str) -> sqlx::Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT id, role as "role: ParticipantRole", name, email FROM "participant"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_participant_communities(&self, participant_id: &Uuid) -> sqlx::Result<Vec<ParticipantCommunity>> {
        sqlx::query_as!(
            ParticipantCommunity,
            r#"
            SELECT * FROM "participant_community"
            WHERE participant_id = $1
            "#,
            participant_id
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_participant_energytransfer(&self, participant_id: &Uuid, community_id: &Uuid, query: EnergyTransferQuery) -> sqlx::Result<Vec<EnergyTransfer>> {
        sqlx::query_as!(
            EnergyTransfer,
            r#"
            SELECT * FROM energytransfer
            WHERE (participant_from = $1 OR participant_to = $1) AND community = $2
            ORDER BY start ASC
            LIMIT $3 OFFSET $4
            "#,
            participant_id,
            community_id,
            TRANSFERS_PER_PAGE as i64,
            (TRANSFERS_PER_PAGE * (query.page - 1)) as i64
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
