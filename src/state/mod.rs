pub(crate) mod data_handler;

use crate::db::DB;
use crate::model::input::InputData;
use crate::model::search::full_text::FullTextSearchResult;
use crate::model::search::vector::VectorSearchResult;
use crate::model::search::{SearchData, SearchModel};
use crate::model::DataModel;
use crate::rank::Rank;
use crate::vo::SelectResultVo;
use futures_util::{stream, StreamExt};
use tracing::error;

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

    /// 数据查询
    /// 1. 文本搜索 -> 分词全文搜索和向量搜索
    /// 2. 图片搜索 -> prompt（文本搜索流程），图片向量搜索
    /// 3. item 搜索 -> 文本搜索和图片搜索
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
                search_ids.extend_from_slice(&Rank::full_text_rank(full_text_result, Some(5))?);
                search_ids.extend_from_slice(&Rank::vector_rank(vector_result, Some(5))?);
                let select_result = self
                    .db
                    .select_by_id(search_ids.into_iter().map(|s| s.id).collect())
                    .await?;
                Ok(select_result
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<SelectResultVo>>())
            }
            SearchModel::Image(image) => {
                // prompt 全文搜索
                let prompt_full_text_result = self
                    .db
                    .full_text_search(image.prompt_search_model.tokens.0)
                    .await?;
                // prompt 向量搜索
                let prompt_vector_result = self
                    .db
                    .vector_search(image.prompt_search_model.vector, None)
                    .await?;
                // 图片向量搜索
                let image_vector_result = self.db.vector_search(image.vector, None).await?;

                let mut search_ids = vec![];
                search_ids
                    .extend_from_slice(&Rank::full_text_rank(prompt_full_text_result, Some(5))?);
                search_ids.extend_from_slice(&Rank::vector_rank(
                    // 合并向量搜索结果
                    image_vector_result
                        .into_iter()
                        .chain(prompt_vector_result.into_iter())
                        .collect::<Vec<VectorSearchResult>>(),
                    Some(5),
                )?);

                let select_result = self
                    .db
                    .select_by_id(search_ids.into_iter().map(|s| s.id).collect())
                    .await?;

                Ok(select_result
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<SelectResultVo>>())
            }
            SearchModel::Item(item) => {
                let mut full_text_result = vec![];
                let mut vector_result = vec![];
                let mut search_ids = vec![];

                stream::iter(item.text)
                    .then(|text| async move {
                        Ok::<(Vec<FullTextSearchResult>, Vec<VectorSearchResult>), anyhow::Error>((
                            self.db.full_text_search(text.tokens.0).await?,
                            self.db.vector_search(text.vector, None).await?,
                        ))
                    })
                    .collect::<Vec<_>>()
                    .await
                    .into_iter()
                    .for_each(|res| match res {
                        Ok(res) => {
                            full_text_result.extend(res.0);
                            vector_result.extend(res.1);
                        }
                        Err(e) => {
                            error!("text search error in search item: {:?}", e);
                        }
                    });

                stream::iter(item.image)
                    .then(|image| async move {
                        let mut vector_result = vec![];
                        vector_result.extend(self.db.vector_search(image.vector, None).await?);
                        vector_result.extend(
                            self.db
                                .vector_search(image.prompt_search_model.vector, None)
                                .await?,
                        );
                        Ok::<(Vec<FullTextSearchResult>, Vec<VectorSearchResult>), anyhow::Error>((
                            self.db
                                .full_text_search(image.prompt_search_model.tokens.0)
                                .await?,
                            vector_result,
                        ))
                    })
                    .collect::<Vec<_>>()
                    .await
                    .into_iter()
                    .for_each(|res| match res {
                        Ok(res) => {
                            full_text_result.extend(res.0);
                            vector_result.extend(res.1);
                        }
                        Err(e) => {
                            error!("image search error in search item: {:?}", e);
                        }
                    });

                search_ids.extend_from_slice(&Rank::full_text_rank(full_text_result, Some(5))?);
                search_ids.extend_from_slice(&Rank::vector_rank(vector_result, Some(5))?);

                let select_result = self
                    .db
                    .select_by_id(search_ids.into_iter().map(|s| s.id).collect())
                    .await?;

                Ok(select_result
                    .into_iter()
                    .map(|s| s.into())
                    .collect::<Vec<SelectResultVo>>())
            }
        }
    }
}

mod test {
    use crate::model::search::{ImageSearchData, ItemSearchData, SearchData};
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
    async fn test_search_text() {
        let state = setup().await;
        let res = state
            .search(SearchData::Text("hello world".to_string().into()))
            .await
            .unwrap();
        println!("search text: {:?}", serde_json::to_string(&res).unwrap());
    }

    #[tokio::test]
    async fn test_search_image() {
        let state = setup().await;
        let image_data = tokio::fs::read("test/image.png").await.unwrap();

        let res = state
            .search(SearchData::Image(ImageSearchData {
                url: "https://example.com".parse().unwrap(),
                data: image_data,
            }))
            .await
            .unwrap();
        println!("search image: {:?}", serde_json::to_string(&res).unwrap());
    }

    #[tokio::test]
    async fn test_search_item() {
        let state = setup().await;
        let image_data = tokio::fs::read("test/image.png").await.unwrap();

        let item = SearchData::Item(ItemSearchData {
            text: vec!["hello world".to_string().into()],
            image: vec![ImageSearchData {
                url: "https://example.com".parse().unwrap(),
                data: image_data,
            }],
        });
        let res = state.search(item).await.unwrap();
        println!("search item: {:?}", serde_json::to_string(&res).unwrap());
    }
}
