use uuid::Uuid;

use crate::error::AppResult;

pub struct Store;

#[allow(unused_variables)]
impl Store {
    pub async fn store_token(&self, token_id: Uuid, participant_id: Uuid) -> AppResult<()> {
        todo!();
    }

    pub async fn is_token_revoked(&self, token_id: Uuid) -> AppResult<bool> {
        todo!();
    }

    pub async fn delete_token(&self, token_id: Uuid) -> AppResult<()> {
        todo!();
    }

    pub async fn delete_all_tokens(&self, participant_id: Uuid) -> AppResult<()> {
        todo!();
    }
}
