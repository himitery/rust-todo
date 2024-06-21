pub mod todo_service {
    use axum::{extract::{Path, State}, http::StatusCode, Json};
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    use crate::module::todo::{
        domain::todo::ActiveModel,
        infra::{
            persistence::repository::todo_repository,
            rest::{
                req::create_todo_req::CreateTodoReq,
                res::todo_res::TodoRes,
                router::AppState,
            },
        },
    };
    use crate::module::todo::infra::rest::req::update_todo_req::UpdateTodoReq;
    use crate::module::todo::infra::rest::res::todo_delete_res::TodoDeleteRes;

    pub async fn list(State(state): State<AppState>) -> (StatusCode, Json<Vec<TodoRes>>) {
        let todos = todo_repository::find_all(&state.conn).await;

        (
            StatusCode::OK,
            Json(todos.iter().map(|todo| TodoRes::from_model(todo)).collect())
        )
    }

    pub async fn create(State(state): State<AppState>, Json(req): Json<CreateTodoReq>) -> (StatusCode, Json<TodoRes>) {
        let todo = todo_repository::save(
            &state.conn,
            ActiveModel {
                title: Set(req.title),
                content: Set(req.content),
                ..Default::default()
            },
        ).await;

        (
            StatusCode::CREATED,
            Json(TodoRes::from_active_model(todo))
        )
    }

    pub async fn update(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<UpdateTodoReq>) -> (StatusCode, Json<Option<TodoRes>>) {
        let todo = todo_repository::find_by_id(&state.conn, id).await;
        match todo.is_none() {
            true => {
                (
                    StatusCode::NOT_FOUND,
                    Json(None)
                )
            }
            false => {
                let mut todo: ActiveModel = todo.unwrap().into();
                if !req.title.is_none() {
                    todo.title = Set(req.title.unwrap());
                }
                if !req.content.is_none() {
                    todo.content = Set(req.content.unwrap());
                }

                let todo = todo_repository::update(&state.conn, todo).await;

                (
                    StatusCode::OK,
                    Json(Option::from(TodoRes::from_model(&todo)))
                )
            }
        }
    }

    pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> (StatusCode, Json<Option<TodoDeleteRes>>) {
        match todo_repository::find_by_id(&state.conn, id).await.is_none() {
            true => {
                (
                    StatusCode::NOT_FOUND,
                    Json(None)
                )
            }
            false => {
                todo_repository::delete(&state.conn, id).await;

                (
                    StatusCode::OK,
                    Json(Option::from(TodoDeleteRes::from_id(id)))
                )
            }
        }
    }
}
