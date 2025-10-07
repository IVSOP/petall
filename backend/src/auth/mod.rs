use serde::{Deserialize, Serialize};

pub mod extractor;
pub mod key_store; 
pub mod oauth; 
pub mod password;
pub mod router;
pub mod session_store;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expiration: chrono::DateTime<chrono::Utc>,
}

impl Session {
    pub fn new_random_from(user_id: uuid::Uuid) -> Self {
        let id = uuid::Uuid::new_v4();
        // TODO: make this configurable ?
        let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
        Self {
            id,
            user_id,
            expiration,
        }
    }
}