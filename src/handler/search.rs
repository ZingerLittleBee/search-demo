use crate::model;
use crate::model::search::{ImageSearchData, ItemSearchData};
use crate::state::AppState;
use crate::vo::result::HTTPResult;
use crate::vo::SelectResultVo;
use axum::extract::State;
use axum::Json;
use futures_util::{stream, StreamExt};
use std::sync::Arc;
use tracing::error;

#[derive(Debug, serde::Deserialize)]
pub struct SearchWithTextParam {
    text: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchWithImageParam {
    url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchWithItemParam {
    text: Vec<String>,
    image: Vec<String>,
}

pub async fn search_with_text(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SearchWithTextParam>,
) -> HTTPResult<SelectResultVo> {
    let search_data = model::search::SearchData::Text(input.text.into());
    let result = state.search(search_data).await;
    match result {
        Ok(result) => HTTPResult {
            status: 200,
            message: None,
            data: Some(result),
        },
        Err(e) => {
            error!("text search error: {e:?}");
            HTTPResult {
                status: 200,
                message: Some(format!("search error: {e:?}")),
                data: None,
            }
        }
    }
}

pub async fn search_with_image(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SearchWithImageParam>,
) -> HTTPResult<SelectResultVo> {
    let image_search_data = match ImageSearchData::from_url(&input.url).await {
        Ok(data) => data,
        Err(e) => {
            error!("invalid image from url: {} with error: {e}", &input.url);
            return HTTPResult {
                status: 400,
                message: Some("parse image error".to_string()),
                data: None,
            };
        }
    };

    let search_data = model::search::SearchData::Image(image_search_data);

    match state.search(search_data).await {
        Ok(result) => HTTPResult {
            status: 200,
            message: None,
            data: Some(result),
        },
        Err(e) => {
            error!("image search error: {e:?}");
            HTTPResult {
                status: 500,
                message: Some(format!("search error: {e:?}")),
                data: None,
            }
        }
    }
}

pub async fn search_with_item(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SearchWithItemParam>,
) -> HTTPResult<SelectResultVo> {
    let search_text_vec = input
        .text
        .iter()
        .map(|text| model::search::TextSearchData(text.clone()))
        .collect();
    let mut search_image_vec = vec![];

    stream::iter(input.image)
        .then(|url_str| async move { ImageSearchData::from_url(&url_str).await })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|res| match res {
            Ok(image_data) => search_image_vec.push(image_data),
            Err(e) => {
                error!("search with item error: {:?}", e);
            }
        });

    let search_data = model::search::SearchData::Item(ItemSearchData {
        text: search_text_vec,
        image: search_image_vec,
    });

    match state.search(search_data).await {
        Ok(result) => HTTPResult {
            status: 200,
            message: None,
            data: Some(result),
        },
        Err(e) => {
            error!("item search error: {e:?}");
            HTTPResult {
                status: 500,
                message: Some(format!("search error: {e:?}")),
                data: None,
            }
        }
    }
}
