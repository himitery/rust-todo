use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDeleteRes {
    id: String,
}


impl TodoDeleteRes {
    pub fn from_id(id: Uuid) -> Self {
        TodoDeleteRes {
            id: id.to_string(),
        }
    }
}