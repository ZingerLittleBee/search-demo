use crate::ai::clip::model::CLIPModel;
use crate::ai::clip::CLIP;
use crate::db::DB;
use crate::model::input_data::InputData;
use std::env;
use std::path::PathBuf;

pub struct DataHandler {
    db: DB,
    clip: CLIP,
}

impl DataHandler {
    pub async fn new(db: DB) -> Self {
        Self {
            db,
            clip: DataHandler::init_clip().await,
        }
    }

    async fn init_clip() -> CLIP {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
        let resource_path = PathBuf::from(manifest_dir).join("resources");

        let uris: [std::path::PathBuf; 3] = CLIPModel::MViTB32.model_uri().into();

        let [image_model_path, text_model_path, text_tokenizer_vocab_path]: [PathBuf; 3] = uris
            .iter()
            .map(|p| resource_path.join(p))
            .collect::<Vec<PathBuf>>()
            .try_into()
            .expect("Failed to convert to array");

        CLIP::new(
            image_model_path,
            text_model_path,
            text_tokenizer_vocab_path,
            CLIPModel::MViTB32,
        )
        .await
        .expect("Failed to load CLIP")
    }

    pub async fn handler_input_data(&self, input_data: InputData) -> anyhow::Result<()> {
        match input_data {
            InputData::Image(image_model) => {}
            InputData::Text(text_model) => {}
            InputData::Item(text_models, image_models) => {}
        }
        Ok(())
    }
}
