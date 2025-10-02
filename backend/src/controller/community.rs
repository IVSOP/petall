use crate::AppState;
use crate::models::db::community::Community;
use crate::models::db::user::{UserCommunity, UserRole};
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

    pub async fn register_user_community(
        &self,
        community: &Uuid,
        user: Uuid,
        role: UserRole,
    ) -> sqlx::Result<UserCommunity> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            INSERT INTO user_community
            (community_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING
            community_id, user_id, role as "role: UserRole"
            "#,
            community,
            user,
            &role as &UserRole
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn remove_user_community(
        &self,
        community: &Uuid,
        user: &Uuid,
    ) -> sqlx::Result<UserCommunity> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            DELETE FROM user_community
            WHERE community_id = $1 AND user_id = $2
            RETURNING
            community_id, user_id, role as "role: UserRole"
            "#,
            community,
            user
        )
        .fetch_one(&self.pg_pool)
        .await
    }

    pub async fn get_community_users(&self, community: &Uuid) -> sqlx::Result<Vec<UserCommunity>> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            SELECT user_id, community_id, role as "role: UserRole" FROM "user_community"
            WHERE community_id = $1
            "#,
            community
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
