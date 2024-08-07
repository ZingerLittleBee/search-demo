mod data_handler;

use crate::db::DB;
use crate::model::input::InputData;
use crate::model::search::{SearchData, SearchModel};
use crate::model::DataModel;

pub struct AppState {
    pub db: DB,
    pub data_handler: data_handler::DataHandler,
}

impl AppState {
    pub async fn new() -> Self {
        let db = DB::new().await;

        Self {
            db,
            data_handler: data_handler::DataHandler::new().await,
        }
    }

    pub async fn data_ingestion(&self, input_data: InputData) -> anyhow::Result<()> {
        match self.data_handler.handle_input_data(input_data).await? {
            DataModel::Text(text) => self.db.insert_text(text).await?,
            DataModel::Image(image) => self.db.insert_image(image).await?,
            DataModel::Item(item) => self.db.insert_item(item).await?,
        }
        Ok(())
    }

    pub async fn search(&self, input: SearchData) -> anyhow::Result<()> {
        match self.data_handler.handle_search_data(input).await? {
            SearchModel::Text(_) => {}
            SearchModel::Image(_) => {}
            SearchModel::Item(_) => {}
        }
        Ok(())
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
