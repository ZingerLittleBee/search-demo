mod data_handler;

use crate::db::DB;
use crate::model::input_data::InputData;

pub struct AppState {
    pub db: DB,
    pub data_handler: data_handler::DataHandler,
}

impl AppState {
    pub async fn new() -> Self {
        let db = DB::new().await;

        Self {
            db: db.clone(),
            data_handler: data_handler::DataHandler::new(db).await,
        }
    }

    pub async fn data_ingestion(&self, input_data: InputData) -> anyhow::Result<()> {
        self.data_handler.handler_input_data(input_data).await
    }
}

mod test {
    use crate::state::AppState;

    #[tokio::test]
    async fn test_new() {
        tracing_subscriber::fmt::init();
        let _app_state = AppState::new().await;
    }
}
