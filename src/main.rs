mod ai;
mod constant;
mod db;
mod handler;
pub mod model;
mod rank;
mod state;
mod vo;
mod utils;

use crate::handler::health_handler;
use crate::handler::inbound::{inbound_image, inbound_item, inbound_text};
use crate::handler::local::{index_handler, static_handler};
use crate::handler::search::{search_with_image, search_with_item, search_with_text};
use crate::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use dotenvy::dotenv;
use handler::get_routes;
use handler::upload::upload_image;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let shared_state = Arc::new(AppState::new().await);

    let app = Router::new()
        .nest("/api", get_routes())
        .route("/", get(index_handler))
        .route("/*file", get(static_handler))
        .with_state(shared_state)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
