use crate::model::search::ID;
use serde::Deserialize;
use surrealdb::sql::Thing;

pub(crate) mod full_text;
pub(crate) mod vector;

impl From<Thing> for ID {
    fn from(value: Thing) -> Self {
        ID::new(value.id.to_string(), value.tb.as_str().into())
    }
}

#[derive(Debug, Deserialize)]
pub struct TextEntity {
    id: Thing,
    data: String,
    vector: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ImageEntity {
    id: Thing,
    url: String,
    prompt: String,
    prompt_vector: Vec<f32>,
    vector: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemEntity {
    id: Thing,
    text: Vec<TextEntity>,
    image: Vec<ImageEntity>,
}

#[derive(Debug, Deserialize)]
pub struct ItemRelationEntity {
    id: Thing,
    r#in: String,
    out: String,
}
