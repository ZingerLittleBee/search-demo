use crate::state::AppState;
use crate::vo::result::HTTPResult;
use axum::extract::Multipart;
use axum::extract::State;
use std::sync::Arc;
use tracing::error;

pub async fn upload_image(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> HTTPResult<Vec<String>> {
    let mut urls = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        match field.bytes().await {
            Ok(data_byte) => {
                let url = state
                    .upload_image(file_name, data_byte.to_vec())
                    .await
                    .unwrap();
                urls.push(url);
            },
            Err(e) => {
                error!("Failed to read field bytes: {}", e);
            }
        }
    }

    HTTPResult {
        status: 200,
        data: Some(urls),
        message: None,
    }
}
