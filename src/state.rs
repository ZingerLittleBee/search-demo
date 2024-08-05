use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
    pub db: DB,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db: DB::new().await,
        }
    }
}
