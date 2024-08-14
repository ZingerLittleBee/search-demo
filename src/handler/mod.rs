use crate::vo::result::HTTPResult;

pub mod search;
pub mod inbound;

pub async fn health_handler() -> HTTPResult<String> {
    HTTPResult {
        status: 200,
        message: None,
        data: None,
    }
}