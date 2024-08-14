mod entity;
mod sql;

use crate::constant::{
    DATABASE_HOST, DATABASE_NAME, DATABASE_NS, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER,
};
use crate::db::entity::full_text::FullTextSearchEntity;
use crate::db::entity::vector::VectorSearchEntity;
use crate::db::entity::{
    ContainRelationEntity, ImageEntity, ItemEntity, SelectResultEntity, TextEntity,
};
use crate::db::sql::CREATE_TABLE;
use crate::model::search::full_text::{FullTextSearchResult, FULL_TEXT_SEARCH_TABLE};
use crate::model::search::vector::{VectorSearchResult, VECTOR_SEARCH_TABLE};
use crate::model::search::{ID, TB};
use crate::model::{ImageModel, ItemModel, TextModel};
use crate::utils::replace_single_quotes;
use futures::future::join_all;
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::debug;

#[derive(Clone)]
pub struct DB {
    client: Surreal<Client>,
}

impl DB {
    pub async fn new() -> Self {
        Self {
            client: DB::init_db().await.expect("Failed to initialize database"),
        }
    }

    async fn init_db() -> anyhow::Result<Surreal<Client>> {
        let db = Surreal::new::<Ws>(format!(
            "{}:{}",
            env::var(DATABASE_HOST)?,
            env::var(DATABASE_PORT)?
        ))
        .await?;
        db.signin(Root {
            username: &env::var(DATABASE_USER)?,
            password: &env::var(DATABASE_PASSWORD)?,
        })
        .await?;
        db.use_ns(env::var(DATABASE_NS)?)
            .use_db(env::var(DATABASE_NAME)?)
            .await?;
        DB::init_table(&db).await?;
        Ok(db)
    }

    async fn init_table(db: &Surreal<Client>) -> anyhow::Result<()> {
        db.query(CREATE_TABLE).await?;
        Ok(())
    }
}

// 📖 入库实现
impl DB {
    pub async fn insert_text(&self, input: TextModel) -> anyhow::Result<()> {
        let mut format_input = input;
        format_input.data = replace_single_quotes(format_input.data.as_str());
        self.client
            .query(
                "
        CREATE text CONTENT {
	        data: $data,
	        vector: $vector
        }",
            )
            .bind(format_input)
            .await?;
        Ok(())
    }

    pub async fn insert_image(&self, input: ImageModel) -> anyhow::Result<()> {
        let mut format_input = input;
        format_input.prompt = replace_single_quotes(format_input.prompt.as_str());
        self.client
            .query(
                "
        CREATE image CONTENT {
	        url: $url,
	        prompt: $prompt,
            vector: $vector,
            prompt_vector: $prompt_vector
        }",
            )
            .bind(format_input)
            .await?;
        Ok(())
    }

    pub async fn insert_item(&self, input: ItemModel) -> anyhow::Result<()> {
        let mut create_text_sql_vec = vec![];
        let mut create_image_sql_vec = vec![];
        let mut item_text_record = vec![];
        let mut item_image_record = vec![];

        input.text.iter().enumerate().for_each(|(i, text)| {
            create_text_sql_vec.push(format!(
                "LET $text_{} = (CREATE ONLY text CONTENT {{data: '{}', vector: [{}]}}).id;",
                i,
                replace_single_quotes(text.data.as_str()),
                text.vector
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
            item_text_record.push(format!("$text_{}", i));
        });
        input.image.iter().enumerate().for_each(|(i, image)| {
            create_image_sql_vec.push(format!(
                "LET $image_{} = (CREATE ONLY image CONTENT {{url: '{}', prompt: '{}', vector: [{}], prompt_vector: [{}]}}).id;",
                i,
                image.url,
                replace_single_quotes(image.prompt.as_str()),
                image
                    .vector
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                image.prompt_vector
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
            item_image_record.push(format!("$image_{}", i));
        });
        let create_item_sql = format!(
            "LET $item = (CREATE ONLY item CONTENT {{ text: [{}], image: [{}]}}).id;",
            item_text_record.join(", "),
            item_image_record.join(", ")
        );
        let create_relate_sql = format!(
            "RELATE {} -> contains -> [{}, {}];",
            "$item",
            item_text_record.join(", "),
            item_image_record.join(", ")
        );

        let sql = [
            create_text_sql_vec.join("\n"),
            create_image_sql_vec.join("\n"),
            create_item_sql,
            create_relate_sql,
        ]
        .join("\n");

        debug!("insert item sql: {}", sql);

        self.client.query(&sql).await?;

        Ok(())
    }
}

// 🔍 全文搜索实现
impl DB {
    pub async fn full_text_search(
        &self,
        data: Vec<String>,
    ) -> anyhow::Result<Vec<FullTextSearchResult>> {
        let futures = FULL_TEXT_SEARCH_TABLE.iter().map(|table| {
            let param_sql = |data: (usize, &String)| -> (String, String) {
                (
                    format!("search::score({}) AS score_{}", data.0, data.0),
                    format!("{} @{}@ '{}'", table.column_name(), data.0, data.1),
                )
            };

            let (search_scores, where_clauses): (Vec<_>, Vec<_>) =
                data.iter().enumerate().map(param_sql).unzip();

            let sql = format!(
                "SELECT id, {} FROM {} WHERE {};",
                search_scores.join(", "),
                table.table_name(),
                where_clauses.join(" AND ")
            );
            debug!("full-text search sql: {}", sql);

            let data = data.clone();
            async move {
                let text: Vec<FullTextSearchEntity> = self.client.query(&sql).await?.take(0)?;
                Ok::<_, anyhow::Error>(
                    text.iter()
                        .map(|t| t.convert_to_result(&data))
                        .collect::<Vec<_>>(),
                )
            }
        });

        let res: Vec<FullTextSearchResult> = join_all(futures)
            .await
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(res)
    }
}

// 🔍 向量搜索实现
impl DB {
    pub async fn vector_search(
        &self,
        data: Vec<f32>,
        range: Option<&str>,
    ) -> anyhow::Result<Vec<VectorSearchResult>> {
        let range = range.unwrap_or_else(|| "<|10,40|>");
        let futures = VECTOR_SEARCH_TABLE.map(|v| {
            let data = data.clone();
            async move {
                let mut res = self
                    .client
                    .query(format!("SELECT id, vector::distance::knn() AS distance FROM {} WHERE {} {} $vector ORDER BY distance;", v.table_name(), v.column_name(), range))
                    .bind(("vector", data))
                    .await?;
                let res: Vec<VectorSearchEntity> = res.take(0)?;
                Ok::<_, anyhow::Error>(res.iter().map(|d| d.into()).collect::<Vec<VectorSearchResult>>())
            }
        });

        let mut res: Vec<VectorSearchResult> = join_all(futures)
            .await
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        res.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(res)
    }
}

// 数据查询
impl DB {
    /// ids: 只包含 text 和 image 表的 ID
    pub async fn select_by_id(&self, ids: Vec<ID>) -> anyhow::Result<Vec<SelectResultEntity>> {
        let mut text_ids = vec![];
        let mut image_ids = vec![];
        let mut item_ids = vec![];

        ids.iter().for_each(|id| match id.tb() {
            TB::Text => text_ids.push(id.id()),
            TB::Image => image_ids.push(id.id()),
            _ => {}
        });

        let outs = text_ids.iter().chain(image_ids.iter()).cloned().collect();
        let relation = self.select_relation_by_out(outs).await?;

        let mut contain_by_item_ids = vec![];

        relation.iter().for_each(|r| {
            item_ids.push(r.in_id());
            contain_by_item_ids.push(r.out_id());
        });

        text_ids = text_ids
            .into_iter()
            .filter(|id| !contain_by_item_ids.contains(&id.to_string()))
            .collect();

        image_ids = image_ids
            .into_iter()
            .filter(|id| !contain_by_item_ids.contains(&id.to_string()))
            .collect();

        let text = self.select_text(text_ids).await?;
        let image = self.select_image(image_ids).await?;
        let item = self.select_item(item_ids).await?;

        let mut res = vec![];
        res.extend(text.into_iter().map(SelectResultEntity::Text));
        res.extend(image.into_iter().map(SelectResultEntity::Image));
        res.extend(item.into_iter().map(SelectResultEntity::Item));
        Ok(res)
    }

    async fn select_relation_by_out(
        &self,
        ids: Vec<impl AsRef<str>>,
    ) -> anyhow::Result<Vec<ContainRelationEntity>> {
        let mut resp = self
            .client
            .query(format!(
                "SELECT * from contains where out in [{}];",
                ids.iter()
                    .map(|i| i.as_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .await?;
        Ok(resp.take::<Vec<ContainRelationEntity>>(0)?)
    }

    async fn select_text(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<TextEntity>> {
        let mut resp = self
            .client
            .query(format!(
                "SELECT * FROM text WHERE id in [{}];",
                ids.iter()
                    .map(|i| i.as_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .await?;
        Ok(resp.take::<Vec<TextEntity>>(0)?)
    }

    async fn select_image(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<ImageEntity>> {
        let mut resp = self
            .client
            .query(format!(
                "SELECT * FROM image WHERE id in [{}];",
                ids.iter()
                    .map(|i| i.as_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .await?;
        Ok(resp.take::<Vec<ImageEntity>>(0)?)
    }

    async fn select_item(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<ItemEntity>> {
        let mut resp = self
            .client
            .query(format!(
                "SELECT * FROM item WHERE id in [{}] FETCH text, image;",
                ids.iter()
                    .map(|i| i.as_ref())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .await?;
        Ok(resp.take::<Vec<ItemEntity>>(0)?)
    }
}

mod test {
    use crate::model::ItemModel;
    use dotenvy::dotenv;
    use rand::Rng;
    use tracing_subscriber::EnvFilter;

    async fn setup() -> crate::db::DB {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        crate::db::DB::new().await
    }

    fn gen_vector() -> Vec<f32> {
        (0..512)
            .map(|_| rand::thread_rng().gen_range(0.0..1.0))
            .collect()
    }

    #[tokio::test]
    async fn test_insert_text() {
        let db = setup().await;
        let text = crate::model::TextModel {
            data: "Hello, World!".to_string(),
            vector: gen_vector(),
        };
        db.insert_text(text).await.unwrap();
        let res = db
            .client
            .query("SELECT * FROM text where data = 'Hello, World!'")
            .await
            .unwrap();
        println!("res: {:?}", res);
    }

    #[tokio::test]
    async fn test_insert_image() {
        let db = setup().await;
        let handler = crate::state::data_handler::DataHandler::new().await;
        let image = crate::model::ImageModel {
            url: "https://example.com".to_string(),
            prompt: "hello world".to_string(),
            vector: gen_vector(),
            prompt_vector: handler.get_text_embedding("hello world").await.unwrap(),
        };
        db.insert_image(image).await.unwrap();
        let res = db
            .client
            .query("SELECT * FROM image where url = 'https://example.com'")
            .await
            .unwrap();
        println!("res: {:?}", res);
    }

    #[tokio::test]
    async fn test_insert_item() {
        let db = setup().await;
        let handler = crate::state::data_handler::DataHandler::new().await;
        let item = ItemModel {
            text: vec![
                crate::model::TextModel {
                    data: "Hello, World!".to_string(),
                    vector: handler.get_text_embedding("Hello, World!").await.unwrap(),
                },
                crate::model::TextModel {
                    data: "Hello, World2!".to_string(),
                    vector: handler.get_text_embedding("Hello, World2!").await.unwrap(),
                },
            ],
            image: vec![
                crate::model::ImageModel {
                    url: "https://example.com".to_string(),
                    prompt: "What is in this picture?".to_string(),
                    vector: gen_vector(),
                    prompt_vector: handler
                        .get_text_embedding("What is in this picture?")
                        .await
                        .unwrap(),
                },
                crate::model::ImageModel {
                    url: "https://example.com2".to_string(),
                    prompt: "What is in this picture2?".to_string(),
                    vector: gen_vector(),
                    prompt_vector: handler
                        .get_text_embedding("What is in this picture2?")
                        .await
                        .unwrap(),
                },
            ],
        };
        db.insert_item(item).await.unwrap();
    }

    #[tokio::test]
    async fn test_full_text_search() {
        let db = setup().await;
        // let handler = crate::state::data_handler::DataHandler::new().await;
        // db.insert_text(crate::model::TextModel {
        //     data: "Rust Web Programming".to_string(),
        //     vector: handler
        //         .get_text_embedding("Rust Web Programming")
        //         .await
        //         .unwrap(),
        // })
        // .await
        // .unwrap();
        // db.insert_text(crate::model::TextModel {
        //     data: "Rust Web Programming2222".to_string(),
        //     vector: handler
        //         .get_text_embedding("Rust Web Programming2222")
        //         .await
        //         .unwrap(),
        // })
        // .await
        // .unwrap();
        // db.insert_text(crate::model::TextModel {
        //     data: "Rust Web Programming3333".to_string(),
        //     vector: handler
        //         .get_text_embedding("Rust Web Programming3333")
        //         .await
        //         .unwrap(),
        // })
        // .await
        // .unwrap();

        let data = vec!["rust".to_string(), "Programming3333".to_string()];
        assert!(db.full_text_search(data).await.unwrap().len() >= 1);
    }

    #[tokio::test]
    async fn test_vector_search() {
        let db = setup().await;
        let handler = crate::state::data_handler::DataHandler::new().await;
        let test_data = "hello world";
        // let test_data2 = "hello world222";
        let embedding_text = handler.get_text_embedding(test_data).await.unwrap();
        // let embedding_text2 = handler.get_text_embedding(test_data2).await.unwrap();
        // db.insert_text(crate::model::TextModel {
        //     data: test_data.to_string(),
        //     vector: embedding_text.clone(),
        // })
        // .await
        // .unwrap();
        // db.insert_text(crate::model::TextModel {
        //     data: test_data2.to_string(),
        //     vector: embedding_text2.clone(),
        // })
        // .await
        // .unwrap();
        println!(
            "vector_search: {:?}",
            db.vector_search(embedding_text, None).await.unwrap()
        );
    }

    #[tokio::test]
    async fn test_select_text() {
        let db = setup().await;
        let ids = vec!["text:kobjmx4b0csfcdr2b2yp", "text:bfqqsxn3oa6ah395tbq5"];
        let res = db.select_text(ids).await.unwrap();
        println!("res: {:?}", res);
        assert!(res.len() >= 2);
    }

    #[tokio::test]
    async fn test_select_image() {
        let db = setup().await;
        let ids = vec!["image:7juby5xev13458xmwaf4", "image:7nlycejva0pbyv9kcgyw"];
        let res = db.select_image(ids).await.unwrap();
        println!("res: {:?}", res);
        assert!(res.len() >= 2);
    }

    #[tokio::test]
    async fn test_select_item() {
        let db = setup().await;
        let ids = vec!["item:im4q7cwxlavqlgl8svgc"];
        let res = db.select_item(ids).await.unwrap();
        println!("res: {:?}", res);
        assert!(res.len() >= 1);
    }

    #[tokio::test]
    async fn test_select_relation() {
        let db = setup().await;
        let ids = vec!["text:kobjmx4b0csfcdr2b2yp"];
        let res = db.select_relation_by_out(ids).await.unwrap();
        println!("res: {:?}", res);
        assert!(res.len() >= 1);
    }

    #[tokio::test]
    async fn test_select_by_id() {
        let db = setup().await;
        let ids = vec![
            crate::model::search::ID::new("text:kobjmx4b0csfcdr2b2yp".to_string(), "text"),
            crate::model::search::ID::new("image:vsiffo113141wrewrwer".to_string(), "image"),
            crate::model::search::ID::new("image:7nlycejva0pbyv9kcgyw".to_string(), "image"),
            crate::model::search::ID::new("image:7juby5xev13458xmwaf4".to_string(), "image"),
        ];
        let res = db.select_by_id(ids).await.unwrap();
        println!("res: {:?}", res);
        println!("res len: {}", res.len())
    }
}
