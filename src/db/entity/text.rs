use crate::model::search::TextSearchResult;
use serde::Deserialize;
use std::collections::HashMap;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub(crate) struct TextEntity {
    id: Thing,
    data: String,
    #[serde(flatten)]
    scores: HashMap<String, f32>,
}

impl TextEntity {
    pub fn convert_to_result(&self, data: Vec<String>) -> TextSearchResult {
        let score = data
            .iter()
            .enumerate()
            .map(|(i, d)| {
                (
                    d.clone(),
                    self.scores
                        .get(&format!("score_{}", i))
                        .unwrap_or(&0.0)
                        .clone(),
                )
            })
            .collect();
        TextSearchResult {
            id: self.id.id.to_string(),
            score,
            data: self.data.clone(),
        }
    }
}
