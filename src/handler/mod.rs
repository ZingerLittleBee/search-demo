use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use inbound::{inbound_image, inbound_item, inbound_text};
use search::{search_with_image, search_with_item, search_with_text};
use upload::upload_image;

use crate::{state::AppState, vo::result::HTTPResult};

pub mod inbound;
pub mod local;
pub mod search;
pub mod upload;

pub async fn health_handler() -> HTTPResult<String> {
    HTTPResult {
        status: 200,
        message: None,
        data: None,
    }
}

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/search/text", post(search_with_text))
        .route("/search/image", post(search_with_image))
        .route("/search/item", post(search_with_item))
        .route("/inbound/text", post(inbound_text))
        .route("/inbound/image", post(inbound_image))
        .route("/inbound/item", post(inbound_item))
        .route("/upload/image", post(upload_image))
}
