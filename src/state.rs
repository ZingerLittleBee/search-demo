use std::{env, path::PathBuf};

use crate::{
    ai::clip::{model::CLIPModel, CLIP},
    db::DB,
};

pub struct AppState {
    pub db: DB,
    pub clip: CLIP,
}

impl AppState {
    pub async fn new() -> Self {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
        let resource_path = PathBuf::from(manifest_dir).join("resources");

        let uris: [std::path::PathBuf; 3] = CLIPModel::MViTB32.model_uri().into();

        let [image_model_path, text_model_path, text_tokenizer_vocab_path]: [PathBuf; 3] = uris
            .iter()
            .map(|p| resource_path.join(p))
            .collect::<Vec<PathBuf>>()
            .try_into()
            .expect("Failed to convert to array");

        let clip = CLIP::new(
            image_model_path,
            text_model_path,
            text_tokenizer_vocab_path,
            CLIPModel::MViTB32,
        )
        .await
        .expect("Failed to load CLIP");

        Self {
            db: DB::new().await,
            clip,
        }
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
