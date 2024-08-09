use crate::model::search::vector::VectorSearchResult;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub(crate) struct VectorSearchEntity {
    id: Thing,
    distance: f32,
}

impl From<VectorSearchEntity> for VectorSearchResult {
    fn from(entity: VectorSearchEntity) -> Self {
        VectorSearchResult {
            id: entity.id.into(),
            distance: entity.distance,
        }
    }
}
