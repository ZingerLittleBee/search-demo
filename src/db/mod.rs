mod sql;

use crate::constant::{
    DATABASE_HOST, DATABASE_NAME, DATABASE_NS, DATABASE_PASSWORD, DATABASE_PORT, DATABASE_USER,
};
use crate::db::sql::CREATE_TABLE;
use crate::model::{ImageModel, ItemModel, TextModel};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

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
        Ok(())
    }
}

mod test {
    use dotenvy::dotenv;

    async fn setup() -> crate::db::DB {
        dotenv().ok();
        crate::db::DB::new().await
    }

    #[tokio::test]
    async fn test_insert_text() {
        let db = setup().await;
        let text = crate::model::TextModel {
            data: "Hello, World!".to_string(),
            vector: vec![0.0, 1.0, 2.0],
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
            vector: vec![0.0, 1.0, 2.0],
        };
        db.insert_image(image).await.unwrap();
        let res = db
            .client
            .query("SELECT * FROM image where url = 'https://example.com'")
            .await
            .unwrap();
        println!("res: {:?}", res);
    }
}
