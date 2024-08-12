pub mod full_text;
pub mod vector;

use crate::model::search::full_text::FullTextSearchResult;
use crate::model::search::vector::VectorSearchResult;
use url::Url;

pub struct TextSearchData(pub String);

pub struct ImageSearchData {
    pub url: Url,
    pub data: Vec<u8>,
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
#[derive(Debug, Clone)]
pub enum TB {
    Text,
    Image,
}

impl From<&str> for TB {
    fn from(value: &str) -> Self {
        match value {
            "text" => TB::Text,
            "image" => TB::Image,
            _ => TB::Text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ID {
    id: String,
    tb: TB,
}

impl ID {
    pub fn new(id: String, tb: &str) -> Self {
        Self { id, tb: tb.into() }
    }

    pub fn table_name(&self) -> &str {
        match self.tb {
            TB::Text => "text",
            TB::Image => "image",
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tb(&self) -> &TB {
        &self.tb
    }
}
