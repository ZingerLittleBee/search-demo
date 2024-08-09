use crate::model::search::ID;

pub struct FullTextSearchResult {
    pub id: ID,
    // 分词，分数
    pub score: Vec<(String, f32)>,
}

pub enum FullTextSearchTable {
    Text,
    Image,
}

impl FullTextSearchTable {
    pub fn table_name(&self) -> &str {
        match self {
            FullTextSearchTable::Text => "text",
            FullTextSearchTable::Image => "image",
        }
    }

    pub fn column_name(&self) -> &str {
        match self {
            FullTextSearchTable::Text => "data",
            FullTextSearchTable::Image => "prompt",
        }
    }
}

pub const FULL_TEXT_SEARCH_TABLE: [FullTextSearchTable; 2] =
    [FullTextSearchTable::Text, FullTextSearchTable::Image];
