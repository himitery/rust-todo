use axum::{
    body::Body,
    http::Request, Router,
};
use dotenv::dotenv;
use tokio;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{debug_span, Level};
use tracing_subscriber;

use crate::module::todo::infra::rest::router::get_todo_router;

mod module;
mod common;

#[tokio::main]
async fn main() {
    // env
    dotenv().ok();

    // tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // database.rs
    let database = common::database::setup().await;
    assert!(database.ping().await.is_ok());

    let router = Router::new()
        .nest("/api/todo", get_todo_router(database).await)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    |request: &Request<Body>| {
                        debug_span!(
                            "request",
                            method = display(request.method()),
                            uri = display(request.uri()),
                        )
                    }
                )
        );

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        router,
    ).await.unwrap()
}
