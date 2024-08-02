use base64::Engine;
use base64::engine::general_purpose;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct RequestData {
    model: String,
    prompt: String,
    stream: bool,
    images: Vec<String>,
}

#[derive(Deserialize)]
struct ResponseData {
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

pub async fn image_to_prompt(images: Vec<String>) -> anyhow::Result<String> {
    let client = Client::new();
    let request_data = RequestData {
        model: "llava".to_string(),
        prompt: "What is in this picture?".to_string(),
        stream: false,
        images,
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request_data)
        .send()
        .await?;

    let response_data: ResponseData = response.json().await?;
    Ok(response_data.response)
}

pub async fn image_to_base64(image_path: impl AsRef<std::path::Path>) -> anyhow::Result<String> {
    let mut file = File::open(image_path).await?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(general_purpose::STANDARD.encode(&buffer))
}

mod test {
    use crate::ai::image_to_prompt::image_to_base64;

    #[tokio::test]
    async fn test_image_to_prompt() {
        let image_base64 = image_to_base64("test/image.png").await.unwrap();

        let response = super::image_to_prompt(vec![image_base64]).await.unwrap();
        println!("image to prompt: {}", response);
    }
}
