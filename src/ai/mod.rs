use serde::{Deserialize};

pub mod clip;
pub mod image_to_prompt;
pub mod translation;

#[derive(Deserialize)]
pub struct ResponseData {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Vec<i32>,
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
    prompt_eval_duration: i64,
    eval_count: i32,
    eval_duration: i64,
}