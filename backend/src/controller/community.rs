use crate::AppState;
use crate::models::db::community::Community;
use crate::models::db::participant::{
    ParticipantRole, ParticipantCommunity
};
use crate::models::http::requests::{
    CommunityRegisterRequest, ParticipantCommunityRegisterRequest,
};
use uuid::Uuid;

impl AppState {
    pub async fn get_communities(&self) -> sqlx::Result<Vec<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_community_by_id(&self, id: &Uuid) -> sqlx::Result<Option<Community>> {
        sqlx::query_as!(
            Community,
            r#"
            SELECT * FROM community
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn register_community(&self, name: &str) -> sqlx::Result<Community> {
        sqlx::query_as!(
            Community,
            r#"
            INSERT INTO community
            (name)
            VALUES ($1)
            RETURNING *
            "#,
            name,
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn register_participant_community(
        &self,
        community: &Uuid,
        participant: Uuid,
        role: ParticipantRole,
    ) -> sqlx::Result<ParticipantCommunity> {
        sqlx::query_as!(
            ParticipantCommunity,
            r#"
            INSERT INTO participant_community
            (community, participant, role)
            VALUES ($1, $2, $3)
            RETURNING
            community, participant, role as "role: ParticipantRole"
            "#,
            community,
            participant,
            &role as &ParticipantRole
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn remove_participant_community(
        &self,
        community: &Uuid,
        participant: &Uuid,
    ) -> sqlx::Result<ParticipantCommunity> {
        sqlx::query_as!(
            ParticipantCommunity,
            r#"
            DELETE FROM participant_community
            WHERE community = $1 AND participant = $2
            RETURNING
            community, participant, role as "role: ParticipantRole"
            "#,
            community,
            participant
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn get_community_participants(
        &self,
        community: &Uuid,
    ) -> sqlx::Result<Vec<ParticipantCommunity>> {
        sqlx::query_as!(
            ParticipantCommunity,
            r#"
            SELECT participant, community, role as "role: ParticipantRole" FROM "participant_community"
            WHERE community = $1
            "#,
            community
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
