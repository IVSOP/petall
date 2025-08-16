use crate::AppState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl AppState {

    pub async fn get_participant_by_id(&self, id: Uuid) -> sqlx::Result<Option<Participant>> {
        Ok(
            sqlx::query_as!(
                Participant,
                r#"
                SELECT id, role as "role: ParticipantRole", name, email FROM "participant"
                WHERE id = $1
                "#,
                id,
            )
            .fetch_optional(&self.pg_pool)
            .await?
        )
    }

    pub async fn _get_participant_by_email(&self, email: &str) -> sqlx::Result<Option<Participant>> {
        Ok(
            sqlx::query_as!(
                Participant,
                r#"
                SELECT id, role as "role: ParticipantRole", name, email FROM "participant"
                WHERE email = $1
                "#,
                email
            )
            .fetch_optional(&self.pg_pool)
            .await?
        )
    }

    pub async fn get_participant_communities(&self, participant_id: Uuid) -> sqlx::Result<Vec<ParticipantCommunity>> {
        Ok(
            sqlx::query_as!(
                ParticipantCommunity,
                r#"
                SELECT * FROM "participant_community"
                WHERE participant_id = $1
                "#,
                participant_id
            )
            .fetch_all(&self.pg_pool)
            .await?,
        )
    }
}
