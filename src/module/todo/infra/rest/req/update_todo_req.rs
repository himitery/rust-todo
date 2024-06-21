use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoReq {
    pub title: Option<String>,
    pub content: Option<String>,
}