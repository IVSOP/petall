use crate::AppState;
use crate::models::db::community::Community;
use crate::models::db::user::{UserCommunity, UserRole};
use uuid::Uuid;

impl AppState {
    pub async fn get_communities_from_user(
        &self,
        user_id: Uuid,
    ) -> sqlx::Result<Vec<(Community, UserRole)>> {
        let rows = sqlx::query!(
            r#"
            SELECT c.id, c.name, c.description, c.image, uc.role as "role: UserRole" FROM community c
            JOIN user_community uc ON c.id = uc.community_id
            WHERE uc.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    Community {
                        id: row.id,
                        name: row.name,
                        description: row.description,
                        image: row.image,
                    },
                    row.role,
                )
            })
            .collect())
    }

    pub async fn create_community(
        &self,
        creator_user_id: Uuid,
        name: &str,
        description: &str,
        image: Option<&str>,
    ) -> sqlx::Result<Community> {
        let mut tx = self.pg_pool.begin().await?;

        let community = sqlx::query_as!(
            Community,
            r#"
            INSERT INTO community
            (name, description, image)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            name,
            description,
            image
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO user_community
            (user_id, community_id, role)
            VALUES ($1, $2, $3)
            "#,
            creator_user_id,
            community.id,
            &UserRole::Manager as &UserRole
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(community)
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
}
