mod ai;
mod db;
mod constant;
pub mod model;
mod state;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::sql::Thing;
use tracing::info;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let shared_state = Arc::new(AppState::new().await);

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
