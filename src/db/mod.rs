mod entity;
pub mod s3;
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
use crate::utils::{deduplicate, escape_single_quotes};
use futures::future::join_all;
use futures_util::{stream, StreamExt};
use std::env;
use anyhow::bail;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::debug;
use tracing::{info, instrument};
use track_macro::expensive_log;

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
    #[expensive_log]
    pub async fn insert_text(&self, input: TextModel) -> anyhow::Result<()> {
        let mut format_input = input;
        format_input.data = escape_single_quotes(format_input.data.as_str());
        self.client
            .query(
                "
        CREATE text CONTENT {
	        data: $data,
	        vector: $vector,
            en_data: $en_data,
            en_vector: $en_vector
        }",
            )
            .bind(format_input)
            .await?;
        Ok(())
    }

    #[expensive_log]
    pub async fn insert_image(&self, input: ImageModel) -> anyhow::Result<()> {
        let mut format_input = input;
        format_input.prompt = escape_single_quotes(format_input.prompt.as_str());
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

    #[expensive_log]
    pub async fn insert_item(&self, input: ItemModel) -> anyhow::Result<()> {
        let mut create_text_sql_vec = vec![];
        let mut create_image_sql_vec = vec![];
        let mut item_text_record = vec![];
        let mut item_image_record = vec![];

        input.text.iter().enumerate().for_each(|(i, text)| {
            create_text_sql_vec.push(format!(
                "LET $text_{} = (CREATE ONLY text CONTENT {{data: '{}', vector: [{}], en_data: '{}', en_vector: [{}] }}).id;",
                i,
                escape_single_quotes(text.data.as_str()),
                text.vector
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                escape_single_quotes(text.en_data.as_str()),
                text.en_vector
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
                escape_single_quotes(image.prompt.as_str()),
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
    #[expensive_log]
    pub async fn full_text_search(
        &self,
        data: Vec<String>,
    ) -> anyhow::Result<Vec<FullTextSearchResult>> {
        if data.is_empty() {
            bail!("data is empty in full text search");
        }
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
                "SELECT id, {} FROM {} WHERE {} LIMIT 100;",
                search_scores.join(", "),
                table.table_name(),
                where_clauses.join(" OR ")
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
    #[expensive_log]
    pub async fn vector_search(
        &self,
        data: Vec<f32>,
        range: Option<&str>,
    ) -> anyhow::Result<Vec<VectorSearchResult>> {
        if data.is_empty() {
            bail!("data is empty in vector search");
        }
        let range = range.unwrap_or_else(|| "<|10,40|>");
        let futures = VECTOR_SEARCH_TABLE.map(|v| {
            let data = data.clone();
            async move {
                let mut res = self
                    .client
                    .query(format!("SELECT id, vector::distance::knn() AS distance FROM {} WHERE {} {} $vector ORDER BY distance LIMIT 100;", v.table_name(), v.column_name(), range))
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
    /// ids 是去重的
    /// 查询出的结果顺序是和 ids 一致的
    pub async fn select_by_id(&self, ids: Vec<ID>) -> anyhow::Result<Vec<SelectResultEntity>> {
        let mut backtrack = vec![];
        stream::iter(ids)
            .then(|id| async move {
                let mut res = vec![];
                let relation = self
                    .select_relation_by_out(vec![id.id()])
                    .await?
                    .into_iter()
                    .map(|r| r.in_id())
                    .collect::<Vec<_>>();
                if !relation.is_empty() {
                    // 有 contain 关系的情况
                    let item = self.select_item(deduplicate(relation)).await?;
                    res.push(
                        item.into_iter()
                            .map(SelectResultEntity::Item)
                            .collect::<Vec<SelectResultEntity>>(),
                    );
                } else {
                    // 没有 contain 关系的情况
                    match id.tb() {
                        TB::Text => {
                            let text = self.select_text(vec![id.id()]).await?;
                            res.push(
                                text.into_iter()
                                    .map(SelectResultEntity::Text)
                                    .collect::<Vec<SelectResultEntity>>(),
                            );
                        }
                        TB::Image => {
                            let image = self.select_image(vec![id.id()]).await?;
                            res.push(
                                image
                                    .into_iter()
                                    .map(SelectResultEntity::Image)
                                    .collect::<Vec<SelectResultEntity>>(),
                            );
                        }
                        _ => {}
                    }
                }
                Ok::<Vec<SelectResultEntity>, anyhow::Error>(res.into_iter().flatten().collect())
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|res| match res {
                Ok(res) => {
                    backtrack.push(res);
                }
                _ => {}
            });

        Ok(backtrack.into_iter().flatten().collect())
    }

    async fn select_relation_by_out(
        &self,
        ids: Vec<impl AsRef<str>>,
    ) -> anyhow::Result<Vec<ContainRelationEntity>> {
        let mut result: Vec<Vec<ContainRelationEntity>> = vec![];
        stream::iter(ids)
            .then(|id| async move {
                let mut resp = self
                    .client
                    .query(format!(
                        "SELECT * from contains where out = {};",
                        id.as_ref()
                    ))
                    .await?;
                let result = resp.take::<Vec<ContainRelationEntity>>(0)?;
                Ok::<_, anyhow::Error>(result)
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|res| match res {
                Ok(relations) => {
                    result.push(relations);
                }
                _ => {}
            });
        Ok(result.into_iter().flatten().collect())
    }

    async fn select_text(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<TextEntity>> {
        let mut result = vec![];

        stream::iter(ids)
            .then(|id| async move {
                let mut resp = self
                    .client
                    .query(format!("SELECT * FROM {};", id.as_ref()))
                    .await?;
                let result = resp.take::<Vec<TextEntity>>(0)?;
                Ok::<_, anyhow::Error>(result)
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|res| match res {
                Ok(image) => {
                    result.push(image);
                }
                _ => {}
            });
        Ok(result.into_iter().flatten().collect())
    }

    async fn select_image(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<ImageEntity>> {
        let mut result = vec![];

        stream::iter(ids)
            .then(|id| async move {
                let mut resp = self
                    .client
                    .query(format!("SELECT * FROM {};", id.as_ref()))
                    .await?;
                let result = resp.take::<Vec<ImageEntity>>(0)?;
                Ok::<_, anyhow::Error>(result)
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|res| match res {
                Ok(image) => {
                    result.push(image);
                }
                _ => {}
            });
        Ok(result.into_iter().flatten().collect())
    }

    async fn select_item(&self, ids: Vec<impl AsRef<str>>) -> anyhow::Result<Vec<ItemEntity>> {
        let mut result = vec![];

        stream::iter(ids)
            .then(|id| async move {
                let mut resp = self
                    .client
                    .query(format!("SELECT * FROM {} FETCH text, image;", id.as_ref()))
                    .await?;
                let result = resp.take::<Vec<ItemEntity>>(0)?;
                Ok::<_, anyhow::Error>(result)
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .for_each(|res| match res {
                Ok(item) => {
                    result.push(item);
                }
                _ => {}
            });

        Ok(result.into_iter().flatten().collect())
    }
}

mod test {
    use dotenvy::dotenv;
    use futures_util::future::join_all;
    use rand::Rng;
    use tracing_subscriber::EnvFilter;
    use crate::db::DB;
    use crate::model::ItemModel;

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
            en_data: "Hello, World!".to_string(),
            en_vector: gen_vector(),
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
                    en_data: "Hello, World!".to_string(),
                    en_vector: gen_vector(),
                },
                crate::model::TextModel {
                    data: "Hello, World2!".to_string(),
                    vector: handler.get_text_embedding("Hello, World2!").await.unwrap(),
                    en_data: "Hello, World!".to_string(),
                    en_vector: gen_vector(),
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
            crate::model::search::ID::new("tkvfpq8o3b0ddkpibo02".to_string(), "text"),
            crate::model::search::ID::new("4wdx1ueb45gjv9ywzxnx".to_string(), "image"),
            // crate::model::search::ID::new("7nlycejva0pbyv9kcgyw".to_string(), "image"),
            // crate::model::search::ID::new("7juby5xev13458xmwaf4".to_string(), "image"),
        ];
        let res = db.select_by_id(ids).await.unwrap();
        println!("res: {:?}", res.iter().map(|r| r.id()).collect::<Vec<_>>());
        println!("res len: {}", res.len())
    }

    #[tokio::test]
    async fn test_batch() {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let search = |db: DB| async move {
            let start = std::time::Instant::now();
            db.client.query("SELECT id, search::score(0), search::score(1) AS score_1 FROM text WHERE data @0@ 'hello' OR data @1@ 'world' limit 100;").await.unwrap();
            // db.client.query("SELECT id, search::score(0) FROM image WHERE prompt @0@ 'image';").await.unwrap();
            // db.client.query("SELECT id FROM text WHERE data @0@ 'hello';").await.unwrap();
            let elapsed = start.elapsed().as_millis();
            println!("(took {} ms)", elapsed);
        };

        println!("1 个");
        let dbs = join_all((0..1).map(|_| DB::new()).collect::<Vec<_>>()).await;
        let search_futures: Vec<_> = dbs.into_iter().map(|db| search(db)).collect();
        let _ = join_all(search_futures).await;

        println!("2 个");
        let dbs = join_all((0..2).map(|_| DB::new()).collect::<Vec<_>>()).await;
        let search_futures: Vec<_> = dbs.into_iter().map(|db| search(db)).collect();
        let _ = join_all(search_futures).await;

        println!("4 个");
        let dbs = join_all((0..4).map(|_| DB::new()).collect::<Vec<_>>()).await;
        let search_futures: Vec<_> = dbs.into_iter().map(|db| search(db)).collect();
        let _ = join_all(search_futures).await;

        println!("10 个");
        let dbs = join_all((0..10).map(|_| DB::new()).collect::<Vec<_>>()).await;
        let search_futures: Vec<_> = dbs.into_iter().map(|db| search(db)).collect();
        let _ = join_all(search_futures).await;

        println!("20 个");
        let dbs = join_all((0..20).map(|_| DB::new()).collect::<Vec<_>>()).await;
        let search_futures: Vec<_> = dbs.into_iter().map(|db| search(db)).collect();
        let _ = join_all(search_futures).await;
    }

    #[tokio::test]
    async fn test_batch_add() {
        let db = setup().await;
        for i in (0..10000) {
            let text = crate::model::TextModel {
                data: format!("Hello, World!{}", i),
                vector: gen_vector(),
                en_data: format!("Hello, World!{}", i),
                en_vector: gen_vector(),
            };
            let start = std::time::Instant::now();
            db.insert_text(text).await.unwrap();
            let elapsed = start.elapsed().as_millis();
            println!("(insert text took {} ms)", elapsed);
        }
    }
}
