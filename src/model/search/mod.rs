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
