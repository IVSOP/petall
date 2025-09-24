use uuid::Uuid;

pub struct Store;

#[allow(unused_variables)]
impl Store {
    pub fn store_token(token_id: Uuid, participant_id: Uuid) {
        todo!();
    }

    pub fn retrieve_token(token_id: Uuid) -> Option<Uuid> {
        todo!();
    }

    pub fn delete_token(token_id: Uuid) {
        todo!();
    }
}
