use serde::{Deserialize, Serialize};

pub mod input;
pub mod search;

#[derive(Serialize, Deserialize)]
pub struct ImageModel {
    pub url: String,
    pub prompt: String,
    pub vector: Vec<f32>,
    pub prompt_vector: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemModel {
    pub text: Vec<TextModel>,
    pub image: Vec<ImageModel>,
}

#[derive(Serialize, Deserialize)]
pub struct TextModel {
    pub data: String,
    pub vector: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub enum DataModel {
    Text(TextModel),
    Image(ImageModel),
    Item(ItemModel),
}
