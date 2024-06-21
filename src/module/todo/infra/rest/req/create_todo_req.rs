use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoReq {
    pub title: String,
    pub content: String,
}