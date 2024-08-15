pub mod full_text;
pub mod vector;

use crate::model::search::full_text::FullTextSearchResult;
use crate::model::search::vector::VectorSearchResult;
use crate::utils::image::load_image_from_url;
use url::Url;

pub struct TextSearchData(pub String);

impl From<String> for TextSearchData {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub struct ImageSearchData {
    pub url: Url,
    pub data: Vec<u8>,
}

impl ImageSearchData {
    pub async fn from_url(url_str: &str) -> anyhow::Result<Self> {
        let url = url_str.parse::<Url>()?;
        let data = load_image_from_url(url.clone()).await?;
        Ok(Self { url, data })
    }
}

pub struct ItemSearchData {
    pub text: Vec<TextSearchData>,
    pub image: Vec<ImageSearchData>,
}

pub enum SearchData {
    Text(TextSearchData),
    Image(ImageSearchData),
    Item(ItemSearchData),
}

pub struct TextToken(pub Vec<String>);

pub struct TextSearchModel {
    pub data: String,
    pub tokens: TextToken,
    pub vector: Vec<f32>,
}

pub struct ImageSearchModel {
    pub url: String,
    pub prompt: String,
    pub prompt_search_model: TextSearchModel,
    /// 图片向量
    pub vector: Vec<f32>,
}

pub struct ItemSearchModel {
    pub text: Vec<TextSearchModel>,
    pub image: Vec<ImageSearchModel>,
}

/// 搜索的入参
pub enum SearchModel {
    Text(TextSearchModel),
    Image(ImageSearchModel),
    Item(ItemSearchModel),
}

pub enum SearchResult {
    Vector(VectorSearchResult),
    FullText(FullTextSearchResult),
}

// table
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub enum TB {
    Text,
    Image,
    Item,
    Frame,
    Video,
}

impl From<&str> for TB {
    fn from(value: &str) -> Self {
        match value {
            "text" => TB::Text,
            "image" => TB::Image,
            "item" => TB::Item,
            "frame" => TB::Frame,
            "video" => TB::Video,
            _ => TB::Text,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ID {
    id: String,
    tb: TB,
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        let mut iter = value.split(':');
        let tb = iter.next().unwrap();
        let id = iter.next().unwrap();
        Self {
            id: id.into(),
            tb: tb.into(),
        }
    }
}

impl ID {
    pub fn new(id: String, tb: &str) -> Self {
        Self { id, tb: tb.into() }
    }

    pub fn table_name(&self) -> &str {
        match self.tb {
            TB::Text => "text",
            TB::Image => "image",
            TB::Item => "item",
            TB::Frame => "frame",
            TB::Video => "video",
        }
    }

    pub fn id(&self) -> String {
        format!("{}:{}", self.table_name(), self.id)
    }

    pub fn tb(&self) -> &TB {
        &self.tb
    }
}
