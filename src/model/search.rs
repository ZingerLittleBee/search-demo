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

pub struct TextSearchModel {
    pub data: String,
    pub vector: Vec<f32>,
}

pub struct ImageSearchModel {
    pub url: String,
    pub prompt: String,
    pub vector: Vec<f32>,
}

pub struct ItemSearchModel {
    pub text: Vec<TextSearchModel>,
    pub image: Vec<ImageSearchModel>,
}

pub enum SearchModel {
    Text(TextSearchModel),
    Image(ImageSearchModel),
    Item(ItemSearchModel),
}
