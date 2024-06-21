pub mod todo_repository {
    use chrono::Utc;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DeleteResult, EntityTrait, Order, QueryOrder};
    use uuid::Uuid;

    use crate::module::todo::domain::todo::{ActiveModel, Column, Entity, Model};

    pub async fn find_all(conn: &DatabaseConnection) -> Vec<Model> {
        Entity::find().order_by(Column::UpdatedAt, Order::Desc).all(conn).await.unwrap_or_else(|err| {
            panic!("[todo_repository::find_all] {}", err.to_string())
        })
    }

    pub async fn find_by_id(conn: &DatabaseConnection, id: Uuid) -> Option<Model> {
        Entity::find_by_id(id).one(conn).await.unwrap_or_else(|err| {
            panic!("[todo_repository::find_by_id] {}", err.to_string())
        })
    }

    pub async fn save(conn: &DatabaseConnection, todo: ActiveModel) -> ActiveModel {
        todo.save(conn).await.unwrap_or_else(|err| {
            panic!("[todo_repository::save] {}", err.to_string())
        })
    }

    pub async fn update(conn: &DatabaseConnection, mut todo: ActiveModel) -> Model {
        todo.updated_at = Set(Utc::now());
        todo.update(conn).await.unwrap_or_else(|err| {
            panic!("[todo_repository::update] {}", err.to_string())
        })
    }

    pub async fn delete(conn: &DatabaseConnection, id: Uuid) -> DeleteResult {
        Entity::delete_by_id(id).exec(conn).await.unwrap_or_else(|err| {
            panic!("[todo_repository::delete] {}", err.to_string())
        })
    }
}