use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TextVo {
    pub id: String,
    pub data: String,
    pub vector: Vec<f32>,
}

#[derive(Debug, Serialize)]
pub struct ImageVo {
    pub id: String,
    pub url: String,
    pub prompt: String,
    pub prompt_vector: Vec<f32>,
    pub vector: Vec<f32>,
}

#[derive(Debug, Serialize)]
pub struct ItemVo {
    pub id: String,
    pub text: Vec<TextVo>,
    pub image: Vec<ImageVo>,
}

#[derive(Debug, Serialize)]
pub enum SelectResultVo {
    Text(TextVo),
    Image(ImageVo),
    Item(ItemVo),
}
