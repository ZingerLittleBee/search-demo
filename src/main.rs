mod ai;
mod db;
mod state;

use crate::state::AppState;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dotenv().ok();

    let shared_state = Arc::new(AppState::new().await);

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
