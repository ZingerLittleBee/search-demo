use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPResult<T: Serialize> {
    #[serde(skip_serializing)]
    pub status: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> IntoResponse for HTTPResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}