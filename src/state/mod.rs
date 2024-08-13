pub(crate) mod data_handler;

use crate::db::DB;
use crate::model::input::InputData;
use crate::model::search::{SearchData, SearchModel};
use crate::model::DataModel;
use crate::rank::{Rank, RankResult};
use crate::vo::SelectResultVo;

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

    // 数据入库
    pub async fn data_ingestion(&self, input_data: InputData) -> anyhow::Result<()> {
        match self.data_handler.handle_input_data(input_data).await? {
            DataModel::Text(text) => self.db.insert_text(text).await?,
            DataModel::Image(image) => self.db.insert_image(image).await?,
            DataModel::Item(item) => self.db.insert_item(item).await?,
        }
        Ok(())
    }

    // 数据查询
    pub async fn search(&self, input: SearchData) -> anyhow::Result<Vec<SelectResultVo>> {
        match self.data_handler.handle_search_data(input).await? {
            SearchModel::Text(text) => {
                let vector = self
                    .data_handler
                    .get_text_embedding(text.data.as_str())
                    .await?;
                let tokens = self.data_handler.tokenizer(text.data.as_str()).await?;
                let full_text_result = self.db.full_text_search(tokens).await?;
                let vector_result = self.db.vector_search(vector, None).await?;
                let mut search_ids = vec![];
                search_ids.extend_from_slice(
                    &Rank::full_text_rank(full_text_result)?
                        .drain(..3)
                        .collect::<Vec<RankResult>>(),
                );
                search_ids.extend_from_slice(
                    &Rank::vector_rank(vector_result)?
                        .drain(..3)
                        .collect::<Vec<RankResult>>(),
                );
                let select_result = self
                    .db
                    .select_by_id(search_ids.into_iter().map(|s| s.id).collect())
                    .await?;
                Ok(select_result
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<SelectResultVo>>())
            }
            SearchModel::Image(_) => Ok(vec![]),
            SearchModel::Item(_) => Ok(vec![]),
        }
    }
}

mod test {
    use crate::model::search::SearchData;
    use crate::state::AppState;
    use dotenvy::dotenv;
    use tracing_subscriber::EnvFilter;

    async fn setup() -> AppState {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        AppState::new().await
    }

    #[tokio::test]
    async fn test_new() {
        setup().await;
    }

    #[tokio::test]
    async fn test_search() {
        let state = setup().await;
        let res = state
            .search(SearchData::Text("hello world".to_string().into()))
            .await
            .unwrap();
        println!("res: {:?}", res);
    }
}
