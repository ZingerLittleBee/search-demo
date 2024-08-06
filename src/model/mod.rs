pub mod input;

pub struct ImageModel {
    pub url: String,
    pub prompt: String,
    pub vector: Vec<f32>,
}

pub struct ItemModel {
    pub text: Vec<TextModel>,
    pub image: Vec<ImageModel>,
}

pub struct TextModel {
    pub data: String,
    pub vector: Vec<f32>,
}

pub enum DataModel {
    Text(TextModel),
    Image(ImageModel),
    Item(ItemModel),
}
