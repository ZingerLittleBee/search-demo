mod sql;

use crate::constant::{
    DATABASE_HOST, DATABASE_NAME, DATABASE_NS, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER,
};
use crate::db::sql::CREATE_TABLE;
use crate::model::search::{ImageSearchModel, ItemSearchModel, TextSearchModel};
use crate::model::{ImageModel, ItemModel, TextModel};
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
        self.client
            .query(
                "
        CREATE text CONTENT {
	        data: $data,
	        vector: $vector
        }",
            )
            .bind(input)
            .await?;
        Ok(())
    }

    pub async fn insert_image(&self, input: ImageModel) -> anyhow::Result<()> {
        self.client
            .query(
                "
        CREATE image CONTENT {
	        url: $url,
	        prompt: $prompt,
            vector: $vector
        }",
            )
            .bind(input)
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
                text.data,
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
                "LET $image_{} = (CREATE ONLY image CONTENT {{url: '{}', prompt: '{}', vector: [{}]}}).id;",
                i,
                image.url,
                image.prompt,
                image
                    .vector
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

// 🔍 搜索实现
impl DB {
    async fn full_text_search(&self, data: String) -> anyhow::Result<()> {
        Ok(())
    }

    async fn vector_search(&self, data: Vec<f32>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn prompt_search(&self, data: String) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn search_text(&self, data: TextSearchModel) -> anyhow::Result<()> {
        // 1. 全文搜索
        // 2. 向量搜索
        // 3. prompt 搜索
        Ok(())
    }

    pub async fn search_image(&self, data: ImageSearchModel) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn search_item(&self, data: ItemSearchModel) -> anyhow::Result<()> {
        Ok(())
    }
}

mod test {
    use crate::model::ItemModel;
    use dotenvy::dotenv;
    use rand::Rng;

    async fn setup() -> crate::db::DB {
        dotenv().ok();
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
        let image = crate::model::ImageModel {
            url: "https://example.com".to_string(),
            prompt: "What is in this picture?".to_string(),
            vector: gen_vector(),
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
        let item = ItemModel {
            text: vec![
                crate::model::TextModel {
                    data: "Hello, World!".to_string(),
                    vector: gen_vector(),
                },
                crate::model::TextModel {
                    data: "Hello, World2!".to_string(),
                    vector: gen_vector(),
                },
            ],
            image: vec![
                crate::model::ImageModel {
                    url: "https://example.com".to_string(),
                    prompt: "What is in this picture?".to_string(),
                    vector: gen_vector(),
                },
                crate::model::ImageModel {
                    url: "https://example.com2".to_string(),
                    prompt: "What is in this picture2?".to_string(),
                    vector: gen_vector(),
                },
            ],
        };
        db.insert_item(item).await.unwrap();
    }
}
