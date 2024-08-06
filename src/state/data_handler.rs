use crate::ai::clip::model::CLIPModel;
use crate::ai::clip::CLIP;
use crate::ai::image_to_prompt::image_to_prompt;
use crate::model::input::{ImageInputData, InputData, ItemInputData, TextInputData};
use crate::model::{ImageModel, ItemModel, DataModel, TextModel};
use futures::future::join_all;
use std::env;
use std::path::PathBuf;

pub struct DataHandler {
    clip: CLIP,
}

impl DataHandler {
    pub async fn new() -> Self {
        Self {
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

    async fn text_input_data_to_model(&self, input: &TextInputData) -> anyhow::Result<TextModel> {
        let vector = self.clip.get_text_embedding(input.0.as_str()).await?;
        Ok(TextModel {
            data: input.0.clone(),
            vector: vector.to_vec(),
        })
    }

    async fn image_input_data_to_model(
        &self,
        input: &ImageInputData,
    ) -> anyhow::Result<ImageModel> {
        let prompt = image_to_prompt(input.data.as_slice()).await?;
        let image = image::load_from_memory(input.data.as_slice())?;
        let vector = self
            .clip
            .get_image_embedding_from_image(&image.to_rgb8())
            .await?;
        Ok(ImageModel {
            url: input.url.clone().into(),
            prompt,
            vector: vector.to_vec(),
        })
    }

    async fn item_input_data_to_model(&self, input: ItemInputData) -> anyhow::Result<ItemModel> {
        let text_future = input
            .text
            .iter()
            .map(|t| async { self.text_input_data_to_model(t).await })
            .collect::<Vec<_>>();
        let image_future = input
            .image
            .iter()
            .map(|i| async { self.image_input_data_to_model(i).await })
            .collect::<Vec<_>>();

        let (text_results, image_results) =
            futures::future::join(join_all(text_future), join_all(image_future)).await;

        Ok(ItemModel {
            text: text_results.into_iter().collect::<Result<Vec<_>, _>>()?,
            image: image_results.into_iter().collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub async fn handler_input_data(&self, input_data: InputData) -> anyhow::Result<DataModel> {
        match input_data {
            InputData::Text(input) => {
                let text_model = self.text_input_data_to_model(&input).await?;
                Ok(DataModel::Text(text_model))
            }
            InputData::Image(input) => {
                let image_model = self.image_input_data_to_model(&input).await?;
                Ok(DataModel::Image(image_model))
            }
            InputData::Item(input) => {
                let item_model = self.item_input_data_to_model(input).await?;
                Ok(DataModel::Item(item_model))
            }
        }
    }
}
