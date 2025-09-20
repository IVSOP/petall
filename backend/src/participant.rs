use crate::AppState;
use crate::router::{EnergyTransferQuery, OrderDirection};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "participant_role", rename_all = "lowercase")]
pub enum ParticipantRole {
    User,
    Manager,
    UserManager,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantCommunity {
    pub participant: Uuid,
    pub community: Uuid,
    pub role: ParticipantRole,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EnergyTransfer {
    pub id: Uuid,
    pub participant: Uuid,
    pub community: Uuid,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub generated: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub consumed: BigDecimal,
    #[serde(with = "bigdecimal::serde::json_num")]
    pub coeficient: BigDecimal,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl AppState {
    pub async fn get_participants(&self) -> sqlx::Result<Vec<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT * FROM "participant"
            "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_participant_by_id(&self, id: &Uuid) -> sqlx::Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT * FROM "participant"
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn _get_participant_by_email(
        &self,
        email: &str,
    ) -> sqlx::Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT * FROM "participant"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_participant_communities(
        &self,
        participant: &Uuid,
    ) -> sqlx::Result<Vec<ParticipantCommunity>> {
        sqlx::query_as!(
            ParticipantCommunity,
            r#"
            SELECT participant, community, role as "role: ParticipantRole" FROM "participant_community"
            WHERE participant = $1
            "#,
            participant
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_participant_energytransfer(
        &self,
        participant_id: &Uuid,
        community_id: &Uuid,
        query: EnergyTransferQuery,
    ) -> sqlx::Result<Vec<EnergyTransfer>> {
        let mut query_builder = QueryBuilder::new(
            r#"
            SELECT id, participant, community, generated, consument, coeficient, start, "end"
            FROM energytransfer
            WHERE (participant_from = "#,
        );

        query_builder.push_bind(participant_id);
        query_builder.push(" OR participant_to = ");
        query_builder.push_bind(participant_id);
        query_builder.push(") AND community = ");
        query_builder.push_bind(community_id);

        if let Some(start) = query.start {
            query_builder.push(" AND start >= ");
            query_builder.push_bind(start);
        }

        if let Some(end) = query.end {
            query_builder.push(" AND \"end\" <= ");
            query_builder.push_bind(end);
        }

        let order_dir = match query.order_dir {
            OrderDirection::Ascending => "ASC",
            OrderDirection::Descending => "DESC",
        };

        query_builder.push(format!(" ORDER BY start {}", order_dir));

        query_builder
            .build_query_as::<EnergyTransfer>()
            .fetch_all(&self.pg_pool)
            .await
    }
}
