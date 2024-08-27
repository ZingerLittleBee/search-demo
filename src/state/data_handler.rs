use crate::ai::clip::model::CLIPModel;
use crate::ai::clip::CLIP;
use crate::ai::image_to_prompt::image_to_prompt;
use crate::constant::STOP_WORDS;
use crate::model::input::{ImageInputData, InputData, ItemInputData, TextInputData};
use crate::model::search::{
    ImageSearchData, ImageSearchModel, ItemSearchData, ItemSearchModel, SearchData, SearchModel,
    TextSearchData, TextSearchModel, TextToken,
};
use crate::model::{DataModel, ImageModel, ItemModel, TextModel};
use anyhow::Ok;
use futures::future::join_all;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use track_macro::expensive_log;
use crate::ai::translation::translate_into_english;
use crate::utils::deduplicate;
use tracing::info;

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

    #[expensive_log]
    pub async fn tokenizer(&self, data: &str) -> anyhow::Result<Vec<String>> {
        let data = data.to_lowercase();
        // 匹配 STOP_WORDS 中的所有单词
        let pattern = format!(r"\b(?:{})\b", STOP_WORDS.join("|"));
        let re_stop_words = Regex::new(&pattern)?;

        // 匹配所有标点符号
        let punctuation_pattern = Regex::new(r"[^\w\s]|[！-～]")?;

        // 移除匹配的停用词
        let cleaned_data = re_stop_words.replace_all(data.as_str(), "");

        // 移除所有标点符号
        let final_result = punctuation_pattern.replace_all(&cleaned_data, "");

        let tokens: Vec<String> = final_result
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(deduplicate(tokens))
    }

    #[expensive_log]
    pub async fn get_text_embedding(&self, data: &str) -> anyhow::Result<Vec<f32>> {
        Ok(self.clip.get_text_embedding(data).await?.to_vec())
    }

    #[expensive_log]
    pub async fn get_image_embedding(&self, data: &[u8]) -> anyhow::Result<Vec<f32>> {
        let image = image::load_from_memory(data)?;
        Ok(self
            .clip
            .get_image_embedding_from_image(&image.to_rgb8())
            .await?
            .to_vec())
    }
}

impl DataHandler {
    #[expensive_log]
    async fn text_input_data_to_model(&self, input: &TextInputData) -> anyhow::Result<TextModel> {
        let vector = self.get_text_embedding(input.0.as_str()).await?;
        let en_data  = translate_into_english(input.0.as_str()).await?;
        let en_vector = self.get_text_embedding(en_data.as_str()).await?;
        Ok(TextModel {
            data: input.0.clone(),
            vector: vector.to_vec(),
            en_data,
            en_vector
        })
    }

    #[expensive_log]
    async fn image_input_data_to_model(
        &self,
        input: &ImageInputData,
    ) -> anyhow::Result<ImageModel> {
        let prompt = image_to_prompt(input.data.as_slice()).await?;
        let image = image::load_from_memory(input.data.as_slice())?;
        let vector = self
            .get_image_embedding(&image.to_rgb8())
            .await?;
        let prompt_vector = self
            .get_text_embedding(prompt.as_str())
            .await?
            .to_vec();
        Ok(ImageModel {
            url: input.url.to_string(),
            prompt,
            vector: vector.to_vec(),
            prompt_vector,
        })
    }

    #[expensive_log]
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

    #[expensive_log]
    pub async fn handle_input_data(&self, input_data: InputData) -> anyhow::Result<DataModel> {
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

    async fn text_search_data_to_model(
        &self,
        input: &TextSearchData,
    ) -> anyhow::Result<TextSearchModel> {
        let vector = self.clip.get_text_embedding(input.0.as_str()).await?;
        Ok(TextSearchModel {
            data: input.0.clone(),
            tokens: TextToken(self.tokenizer(input.0.as_str()).await?),
            vector: vector.to_vec(),
        })
    }
}

impl DataHandler {
    async fn image_search_data_to_model(
        &self,
        input: &ImageSearchData,
    ) -> anyhow::Result<ImageSearchModel> {
        let prompt = image_to_prompt(input.data.as_slice()).await?;
        self.get_image_embedding(input.data.as_slice()).await?;
        Ok(ImageSearchModel {
            url: input.url.to_string(),
            prompt: prompt.clone(),
            prompt_search_model: TextSearchModel {
                data: prompt.clone(),
                tokens: TextToken(self.tokenizer(prompt.as_str()).await?),
                vector: self
                    .clip
                    .get_text_embedding(prompt.as_str())
                    .await?
                    .to_vec(),
            },
            vector: self.get_image_embedding(input.data.as_slice()).await?,
        })
    }

    async fn item_search_data_to_model(
        &self,
        input: &ItemSearchData,
    ) -> anyhow::Result<ItemSearchModel> {
        let text_future = input
            .text
            .iter()
            .map(|t| async { self.text_search_data_to_model(t).await })
            .collect::<Vec<_>>();
        let image_future = input
            .image
            .iter()
            .map(|i| async { self.image_search_data_to_model(i).await })
            .collect::<Vec<_>>();

        let (text_results, image_results) =
            futures::future::join(join_all(text_future), join_all(image_future)).await;

        Ok(ItemSearchModel {
            text: text_results.into_iter().collect::<Result<Vec<_>, _>>()?,
            image: image_results.into_iter().collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub async fn handle_search_data(&self, input: SearchData) -> anyhow::Result<SearchModel> {
        match input {
            SearchData::Text(text) => Ok(SearchModel::Text(
                self.text_search_data_to_model(&text).await?,
            )),
            SearchData::Image(image) => Ok(SearchModel::Image(
                self.image_search_data_to_model(&image).await?,
            )),
            SearchData::Item(item) => Ok(SearchModel::Item(
                self.item_search_data_to_model(&item).await?,
            )),
        }
    }
}

mod test {
    #[tokio::test]
    async fn test_tokenizer() {
        let data_handler = super::DataHandler::new().await;
        let tokens = data_handler.tokenizer("hello world!").await.unwrap();
        let tokens2 = data_handler.tokenizer("There are many people stopping and looking at the seaside. Children are playing on the beach").await.unwrap();
        let tokens3 = data_handler.tokenizer("I'm a teacher").await.unwrap();
        println!("tokens: {:?}", tokens);
        println!("tokens2: {:?}", tokens2);
        println!("tokens3: {:?}", tokens3);
        assert_eq!(tokens, vec!["hello", "world"]);
    }
}
