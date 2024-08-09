use crate::model::search::ID;
use surrealdb::sql::Thing;

pub(crate) mod full_text;
pub(crate) mod vector;

impl From<Thing> for ID {
    fn from(value: Thing) -> Self {
        ID::new(value.id.to_string(), value.tb.as_str().into())
    }
}
