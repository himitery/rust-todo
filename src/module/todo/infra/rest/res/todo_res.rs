use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::module::todo::domain::todo::{ActiveModel, Model};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoRes {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    content: String,
}


impl TodoRes {
    pub fn from_model(todo: &Model) -> Self {
        TodoRes {
            id: todo.id.to_string(),
            created_at: todo.created_at,
            updated_at: todo.updated_at,
            title: todo.title.to_string(),
            content: todo.content.to_string(),
        }
    }

    pub fn from_active_model(todo: ActiveModel) -> Self {
        TodoRes {
            id: todo.id.unwrap().to_string(),
            created_at: todo.created_at.unwrap(),
            updated_at: todo.updated_at.unwrap(),
            title: todo.title.unwrap(),
            content: todo.content.unwrap(),
        }
    }
}