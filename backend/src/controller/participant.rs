use crate::AppState;
use crate::models::db::community::Energy;
use crate::models::db::participant::{Participant, ParticipantCommunity, ParticipantRole};
use crate::models::http::requests::{EnergyQuery, OrderDirection};
use sqlx::QueryBuilder;
use uuid::Uuid;

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

    pub async fn get_participant_by_id(
        &self,
        participant_id: &Uuid,
    ) -> sqlx::Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT * FROM "participant"
            WHERE id = $1
            "#,
            participant_id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_participant_by_email(&self, email: &str) -> sqlx::Result<Option<Participant>> {
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

    pub async fn get_participant_energies(
        &self,
        participant_id: &Uuid,
        community_id: &Uuid,
        query: EnergyQuery,
    ) -> sqlx::Result<Vec<Energy>> {
        let mut query_builder = QueryBuilder::new(
            r#"
            SELECT id, participant, community, generated, consumed, coeficient, start, "end"
            FROM energypool
            WHERE participant = "#,
        );

        query_builder.push_bind(participant_id);
        query_builder.push(" AND community = ");
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
        query_builder.push(format!(
            " LIMIT {} OFFSET {}",
            query.size,
            (query.page - 1) * query.size
        ));

        query_builder
            .build_query_as::<Energy>()
            .fetch_all(&self.pg_pool)
            .await
    }
}
