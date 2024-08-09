use crate::model::search::ID;

#[derive(Debug)]
pub struct VectorSearchResult {
    pub id: ID,
    pub distance: f32,
}

pub enum VectorSearchTable {
    Text,
    Image,
    ImagePrompt,
}

impl VectorSearchTable {
    pub fn table_name(&self) -> &str {
        match self {
            VectorSearchTable::Text => "text",
            VectorSearchTable::Image => "image",
            VectorSearchTable::ImagePrompt => "image",
        }
    }

    pub fn column_name(&self) -> &str {
        match self {
            VectorSearchTable::Text => "vector",
            VectorSearchTable::Image => "vector",
            VectorSearchTable::ImagePrompt => "prompt_vector",
        }
    }
}

pub const VECTOR_SEARCH_TABLE: [VectorSearchTable; 3] = [
    VectorSearchTable::Text,
    VectorSearchTable::Image,
    VectorSearchTable::ImagePrompt,
];
