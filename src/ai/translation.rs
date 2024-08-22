use std::env;
use crate::ai::ResponseData;
use base64::engine::general_purpose;
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use crate::constant::{OLLAMA_ENDPOINT, S3_ENDPOINT};

pub async fn translate_into_english(words: &str) -> anyhow::Result<String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/api/generate", env::var(OLLAMA_ENDPOINT)?))
        .json(&json!({
            "model": "qwen2:7b-instruct-q4_0",
            "stream": false,
            "prompt": format!(r#"Please translate user input into English.
            Please response with translated text only, do not contain anything else.
            If input is English, response with input directly.
            The following text between ``` is user's input.
            ```
            {}
            ```"#, words),
        }))
        .send()
        .await?;
    
    let response_data: ResponseData = response.json().await?;
    Ok(response_data.response)
}

mod test {
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_translate_into_english() {
        dotenv().ok();
        let words = "你好";
        let words2 = "什么是 gpt";
        let res = crate::ai::translation::translate_into_english(words).await.unwrap();
        let res2 = crate::ai::translation::translate_into_english(words2).await.unwrap();
        assert_eq!(res.to_lowercase(), "hello".to_string());
        assert_eq!(res2.to_lowercase(), "what is gpt".to_string());
    }
}