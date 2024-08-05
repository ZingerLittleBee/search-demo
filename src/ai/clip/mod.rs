pub mod model;
mod utils;

use std::path::{Path, PathBuf};

use anyhow::anyhow;
use image::RgbImage;
use ndarray::{Array1, Axis};
use ort::Session;
use tokenizers::tokenizer::Tokenizer;
use utils::{load_onnx_model, normalize};
mod preprocess;

pub struct CLIP {
    image_model: Option<Session>,
    text_model: Option<Session>,
    text_tokenizer: Option<Tokenizer>,
    dim: usize,
}

type CLIPEmbedding = Array1<f32>;

#[derive(Clone)]
pub enum CLIPInput {
    Image(RgbImage),
    ImageFilePath(PathBuf),
    Text(String),
}

impl CLIP {
    pub async fn new(
        image_model_path: impl AsRef<Path>,
        text_model_path: impl AsRef<Path>,
        text_tokenizer_vocab_path: impl AsRef<Path>,
        model_type: model::CLIPModel,
    ) -> anyhow::Result<Self> {
        // let (image_model_uri, text_model_uri, text_tokenizer_vocab_uri) = model_type.model_uri();
        let dim = model_type.dim();

        // let download = file_downloader::FileDownload::new(file_downloader::FileDownloadConfig {
        //     resources_dir: resources_dir.as_ref().to_path_buf(),
        //     ..Default::default()
        // });

        // let image_model_path = download.download_if_not_exists(&image_model_uri).await?;
        // let text_model_path = download.download_if_not_exists(&text_model_uri).await?;
        // let text_tokenizer_vocab_path = download
        //     .download_if_not_exists(&text_tokenizer_vocab_uri)
        //     .await?;

        Self::from_file(
            image_model_path,
            text_model_path,
            text_tokenizer_vocab_path,
            dim,
        )
    }

    pub fn from_file(
        image_model_path: impl AsRef<Path>,
        text_model_path: impl AsRef<Path>,
        text_tokenizer_vocab_path: impl AsRef<Path>,
        dim: usize,
    ) -> anyhow::Result<Self> {
        let image_model = load_onnx_model(image_model_path, None)?;
        let text_model = load_onnx_model(text_model_path, None)?;

        let text_tokenizer = match Tokenizer::from_file(text_tokenizer_vocab_path) {
            Ok(mut tokenizer) => {
                let truncation = tokenizers::utils::truncation::TruncationParams {
                    // default CLIP text truncation
                    max_length: 77,
                    ..Default::default()
                };
                tokenizer.with_truncation(Some(truncation)).ok();

                Some(tokenizer)
            }
            _ => None,
        };

        Ok(Self {
            image_model: Some(image_model),
            text_model: Some(text_model),
            text_tokenizer,
            dim,
        })
    }

    /// Preprocess image and get embedding (in size 1 * DIM)
    ///
    /// # Arguments
    ///
    /// * `image_path` - input image path
    pub async fn get_image_embedding_from_file(
        &self,
        image_path: impl AsRef<Path>,
    ) -> anyhow::Result<CLIPEmbedding> {
        let image_data = tokio::fs::read(image_path.as_ref().to_path_buf()).await?;
        let image = image::load_from_memory(image_data.to_vec().as_slice())?;
        self.get_image_embedding_from_image(&image.to_rgb8()).await
    }

    pub async fn get_image_embedding_from_image(
        &self,
        image: &RgbImage,
    ) -> anyhow::Result<CLIPEmbedding> {
        let image_model = self
            .image_model
            .as_ref()
            .ok_or(anyhow!("image model not found"))?;

        let image = preprocess::preprocess_rgb8_image(image)?;

        // add axis to reshape to (1, C, H, W)
        let image = image.insert_axis(Axis(0)).clone();
        let outputs = image_model.run(ort::inputs!["pixel_values" => image.view()]?)?;

        let output = outputs
            .get("output")
            .ok_or(anyhow!("output not found"))?
            .try_extract_tensor::<f32>()?
            .view()
            .to_owned();

        let output: CLIPEmbedding = output.into_shape(self.dim)?.into_dimensionality()?;

        Ok(normalize(output))
    }

    pub async fn get_text_embedding(&self, text: &str) -> anyhow::Result<CLIPEmbedding> {
        let model = self
            .text_model
            .as_ref()
            .ok_or(anyhow!("text model not found"))?;
        let tokenizer = self
            .text_tokenizer
            .as_ref()
            .ok_or(anyhow!("text tokenizer not found"))?;

        let encoding = tokenizer.encode(text, true).map_err(|err| anyhow!(err))?;

        let ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();
        let ids = ndarray::arr1(&ids).mapv(|x| x as i64);
        let attention_mask = ndarray::arr1(&attention_mask).mapv(|x| x as i64);
        // add axis
        let ids = ids.insert_axis(Axis(0)).clone();
        let attention_mask = attention_mask.insert_axis(Axis(0)).clone();

        let outputs = model.run(
            ort::inputs!["input_ids" => ids.view(), "attention_mask" => attention_mask.view()]?,
        )?;

        let output = outputs
            .get("sentence_embedding")
            .ok_or(anyhow!("output not found"))?
            .try_extract_tensor::<f32>()?
            .view()
            .to_owned();

        let output: CLIPEmbedding = output.into_shape(self.dim)?.into_dimensionality()?;

        Ok(normalize(output))
    }

    pub fn dim(&self) -> usize {
        self.dim
    }
}
