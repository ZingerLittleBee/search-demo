use crate::vo::result::HTTPResult;

pub mod inbound;
pub mod search;
pub mod upload;

pub async fn health_handler() -> HTTPResult<String> {
    HTTPResult {
        status: 200,
        message: None,
        data: None,
    }
}
