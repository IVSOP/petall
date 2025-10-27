use serde::Serialize;
use uuid::Uuid;

use crate::{
    AppState,
    error::AppResult,
    models::{Community, User},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminListCommunityView {
    #[serde(flatten)]
    pub community: Community,
    pub user_count: u32,
}

impl AppState {
    /// Determines if a user has access to the admin view in the frontend.
    /// Users can access the admin view if they are either:
    /// - An admin (with access to all communities)
    /// - A community manager for at least one community
    pub async fn can_access_admin_view(&self, user: &User) -> AppResult<bool> {
        if user.is_admin {
            return Ok(true);
        }

        let count = sqlx::query!(
            "SELECT COUNT(*) FROM community_manager WHERE user_id = $1",
            user.id
        )
        .fetch_one(&self.pg_pool)
        .await?
        .count
        .unwrap_or(0);

        Ok(count > 0)
    }

    pub async fn list_admin_community_view(
        &self,
        user_id: Uuid,
        is_admin: bool,
    ) -> AppResult<Vec<AdminListCommunityView>> {
        let communities = if is_admin {
            sqlx::query_as!(
                Community,
                "
                SELECT id, name, description, image FROM community
                "
            )
            .fetch_all(&self.pg_pool)
            .await?
        } else {
            sqlx::query_as!(
                Community,
                "
                SELECT c.id, c.name, c.description, c.image FROM community c
                JOIN community_manager cm ON c.id = cm.community_id
                WHERE cm.user_id = $1
                ",
                user_id
            )
            .fetch_all(&self.pg_pool)
            .await?
        };

        let mut result = Vec::new();

        for community in communities {
            let user_count = sqlx::query!(
                "SELECT COUNT(*) FROM community_user WHERE community_id = $1",
                community.id
            )
            .fetch_one(&self.pg_pool)
            .await?
            .count
            .unwrap_or(0) as u32;

            result.push(AdminListCommunityView {
                community,
                user_count,
            });
        }

        Ok(result)
    }
}
