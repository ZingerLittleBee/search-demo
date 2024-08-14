use crate::utils::image::load_image_from_url;
use url::Url;

pub struct TextInputData(pub String);

impl From<String> for TextInputData {
    fn from(text: String) -> Self {
        Self(text)
    }
}

pub struct ImageInputData {
    pub url: Url,
    pub data: Vec<u8>,
}

impl ImageInputData {
    pub async fn from_url(url_str: &str) -> anyhow::Result<Self> {
        let url = url_str.parse::<Url>()?;
        let data = load_image_from_url(url.clone()).await?;
        Ok(Self { url, data })
    }
}

pub struct ItemInputData {
    pub text: Vec<TextInputData>,
    pub image: Vec<ImageInputData>,
}

impl From<(Vec<TextInputData>, Vec<ImageInputData>)> for ItemInputData {
    fn from(value: (Vec<TextInputData>, Vec<ImageInputData>)) -> Self {
        Self {
            text: value.0,
            image: value.1,
        }
    }
}

pub enum InputData {
    Text(TextInputData),
    Image(ImageInputData),
    Item(ItemInputData),
}
