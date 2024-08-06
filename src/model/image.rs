use url::Url;

pub struct ImageModel {
    pub url: Url,
    pub prompt: String,
    pub vector: Vec<f32>,
}
