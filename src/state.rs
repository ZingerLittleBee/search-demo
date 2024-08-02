use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            db: AppState::init_db()
                .await
                .expect("Failed to initialize database"),
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
        Ok(db)
    }
}
