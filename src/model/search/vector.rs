use crate::model::search::ID;

pub struct VectorSearchResult {
    pub id: ID,
    pub distance: f32,
}
