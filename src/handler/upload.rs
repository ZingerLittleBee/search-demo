use crate::state::AppState;
use crate::vo::result::HTTPResult;
use axum::extract::Multipart;
use axum::extract::State;
use std::sync::Arc;

pub async fn upload_image(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> HTTPResult<()> {
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => {
                return HTTPResult::error(400, format!("Failed to read multipart field: {}", e))
            }
        };

        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return HTTPResult::error(400, "Field name is missing".to_string()),
        };

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => return HTTPResult::error(400, format!("Failed to read field data: {}", e)),
        };

        if let Err(e) = state.upload_image(name, data.to_vec()).await {
            return HTTPResult::error(500, format!("Failed to upload image: {}", e));
        }
    }
    HTTPResult::success()
}
