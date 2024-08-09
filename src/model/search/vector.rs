use crate::model::search::ID;

#[derive(Debug)]
pub struct VectorSearchResult {
    pub id: ID,
    pub distance: f32,
}

pub enum VectorSearchTable {
    Text,
    Image,
}

impl VectorSearchTable {
    pub fn table_name(&self) -> &str {
        match self {
            VectorSearchTable::Text => "text",
            VectorSearchTable::Image => "image",
        }
    }

    pub fn column_name(&self) -> &str {
        match self {
            VectorSearchTable::Text => "vector",
            VectorSearchTable::Image => "vector",
        }
    }
}

pub const VECTOR_SEARCH_TABLE: [VectorSearchTable; 2] =
    [VectorSearchTable::Text, VectorSearchTable::Image];
