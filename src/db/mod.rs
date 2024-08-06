mod sql;

use crate::db::sql::CREATE_ITEM_TABLE;
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
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("dam").use_db("search").await?;
        DB::init_content_table(&db).await?;
        Ok(db)
    }

    async fn init_content_table(db: &Surreal<Client>) -> anyhow::Result<()> {
        db.query(CREATE_ITEM_TABLE).await?;

        db.query(r#"
        -- 插入多条文本记录
CREATE text SET data = 'Text 1', vector = [0.1, 0.2, 0.3];
CREATE text SET data = 'Text 2', vector = [0.4, 0.5, 0.6];

-- 插入多条图像记录
CREATE image SET url = 'https://example.com/image1.jpg', vector = [0.7, 0.8, 0.9], prompt = 'Image 1';
CREATE image SET url = 'https://example.com/image2.jpg', vector = [1.0, 1.1, 1.2], prompt = 'Image 2';

-- 在 "item" 表中插入一条记录，并建立与多条 "text" 和 "image" 记录的关联关系
LET $texts = (SELECT id FROM text WHERE data IN ['Text 1', 'Text 2']);
LET $images = (SELECT id FROM image WHERE url IN ['https://example.com/image1.jpg', 'https://example.com/image2.jpg']);

CREATE item SET
  text = (RELATE $texts),
  image = (RELATE $images);
        "#).await?;

        Ok(())
    }
}
