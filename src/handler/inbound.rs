use crate::model::input::{ImageInputData, InputData};
use crate::state::AppState;
use crate::vo::result::HTTPResult;
use axum::extract::State;
use axum::Json;
use futures_util::{stream, StreamExt};
use std::sync::Arc;
use tracing::error;

#[derive(Debug, serde::Deserialize)]
pub struct InboundTextParam {
    text: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct InboundImageParam {
    url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct InboundItemParam {
    text: Vec<String>,
    image: Vec<String>,
}

pub async fn inbound_text(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InboundTextParam>,
) -> HTTPResult<()> {
    match state
        .data_ingestion(InputData::Text(input.text.into()))
        .await
    {
        Ok(_) => HTTPResult {
            status: 200,
            message: None,
            data: None,
        },
        Err(e) => {
            error!("text ingestion error: {e:?}");
            HTTPResult {
                status: 500,
                message: Some(format!("ingestion error: {e:?}")),
                data: None,
            }
        }
    }
}

pub async fn inbound_image(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InboundImageParam>,
) -> HTTPResult<()> {
    let image_input_data = match ImageInputData::from_url(&input.url).await {
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

    match state
        .data_ingestion(InputData::Image(image_input_data))
        .await
    {
        Ok(_) => HTTPResult {
            status: 200,
            message: None,
            data: None,
        },
        Err(e) => {
            error!("image ingestion error: {e:?}");
            HTTPResult {
                status: 500,
                message: Some(format!("ingestion error: {e:?}")),
                data: None,
            }
        }
    }
}

pub async fn inbound_item(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InboundItemParam>,
) -> HTTPResult<()> {
    let text_input_data_vec = input.text.iter().map(|t| t.clone().into()).collect();
    let mut image_input_data_vec = vec![];

    stream::iter(input.image)
        .then(|url| async move { ImageInputData::from_url(&url).await })
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|data| {
            match data {
                Ok(d) => {
                    image_input_data_vec.push(d);
                }
                Err(e) => {
                    error!("inbound item error: {e:?}");
                }
            };
        });

    match state
        .data_ingestion(InputData::Item(
            (text_input_data_vec, image_input_data_vec).into(),
        ))
        .await
    {
        Ok(_) => HTTPResult {
            status: 200,
            message: None,
            data: None,
        },
        Err(e) => {
            error!("item ingestion error: {e:?}");
            HTTPResult {
                status: 500,
                message: Some(format!("ingestion error: {e:?}")),
                data: None,
            }
        }
    }
}
