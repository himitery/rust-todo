use axum::{Router, routing::{delete, get, patch, post}};
use sea_orm::DatabaseConnection;

use crate::module::todo::application::service::todo_service;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn get_todo_router(conn: DatabaseConnection) -> Router {
    Router::new()
        .route("/list", get(todo_service::list))
        .route("/new", post(todo_service::create))
        .route("/:id", patch(todo_service::update))
        .route("/:id", delete(todo_service::delete))
        .with_state(AppState { conn })
}