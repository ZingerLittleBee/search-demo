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

/// 文本的搜索结果
pub struct TextSearchResult {
    // 命中记录的 ID
    pub id: String,
    // 命中记录的结果
    pub data: String,
    // 搜索的关键词，分数
    pub score: Vec<(String, f32)>,
}

/// 图片的搜索结果
pub struct ImageSearchResult {
    pub id: String,
}

/// 搜索的结果
pub enum SearchResult {
    Text(Vec<TextSearchResult>),
    Image(Vec<ImageSearchResult>),
}
