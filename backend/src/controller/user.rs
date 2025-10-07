use crate::error::AppResult;
use crate::models::{Key, User, UserCommunity, UserRole};
use crate::{AppState, auth};
use uuid::Uuid;

impl AppState {
    pub async fn get_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            "#
        )
        .fetch_all(&self.pg_pool)
        .await
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE id = $1
            "#,
            user_id,
        )
        .fetch_optional(&self.pg_pool)
        .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(Into::into)
    }

    pub async fn register_user(&self, email: &str, name: &str, password: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (email, name)
            VALUES ($1, $2)
            RETURNING *
            "#,
            email,
            name
        )
        .fetch_one(&self.pg_pool)
        .await?;

        // create the password for this users email
        let hashed_password = auth::password::hash_password(password)?;
        let key_id = Key::email_key_id(email);

        self.create_key(&key_id, user.id, Some(hashed_password))
            .await?;

        Ok(user)
    }

    // when user changes password
    pub async fn update_user_password(
        &self,
        _user_id: &Uuid,
        email: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let hashed_password = auth::password::hash_password(new_password)?;
        let key_id = Key::email_key_id(email);

        self.update_key_password(&key_id, hashed_password).await
    }

    pub async fn get_user_communities(&self, user: &Uuid) -> sqlx::Result<Vec<UserCommunity>> {
        sqlx::query_as!(
            UserCommunity,
            r#"
            SELECT user_id, community_id, role as "role: UserRole" FROM "user_community"
            WHERE user_id = $1
            "#,
            user
        )
        .fetch_all(&self.pg_pool)
        .await
    }
}
