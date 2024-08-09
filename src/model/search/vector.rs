pub struct TextVectorResult {
    pub id: String,
    pub distance: f32,
    pub data: String,
}

pub struct ImageVectorResult {
    pub id: String,
    pub distance: f32,
    pub url: String,
    pub prompt: String,
}

pub enum VectorSearchResult {
    Text(Vec<TextVectorResult>),
    Image(Vec<ImageVectorResult>),
}
