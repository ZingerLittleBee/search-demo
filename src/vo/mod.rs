pub mod result;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TextVo {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct ImageVo {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ItemVo {
    pub id: String,
    pub text: Vec<TextVo>,
    pub image: Vec<ImageVo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct SelectResultVo {
    pub text: Vec<TextVo>,
    pub image: Vec<ImageVo>,
    pub item: Vec<ItemVo>,
}
