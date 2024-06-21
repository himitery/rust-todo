use std::env;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn setup() -> DatabaseConnection {
    Database::connect(
        ConnectOptions::new(env::var("DATABASE_URL").expect("DATABASE_URL is required."))
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .clone()
    ).await.expect("Failed to create database connection")
}