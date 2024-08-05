use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ImageInput {
    base64_images: Vec<String>,
}

#[derive(Serialize)]
struct TextInput {
    texts: Vec<String>,
}

#[derive(Deserialize)]
struct VectorResponse {
    vectors: Vec<Vec<f32>>,
}

struct VectorApi {
    base_url: String,
    client: Client,
}

impl VectorApi {
    fn new(base_url: String) -> Self {
        VectorApi {
            base_url,
            client: Client::new(),
        }
    }

    async fn image_to_vector(&self, base64_images: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/image_to_vector", self.base_url);
        let input = ImageInput { base64_images };

        let response = self
            .client
            .post(&url)
            .json(&input)
            .send()
            .await?
            .json::<VectorResponse>()
            .await?;

        Ok(response.vectors)
    }

    async fn text_to_vector(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/text_to_vector", self.base_url);
        let input = TextInput { texts };

        let response = self
            .client
            .post(&url)
            .json(&input)
            .send()
            .await?
            .json::<VectorResponse>()
            .await?;

        Ok(response.vectors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_URL: &str = "http://localhost:8080";

    #[tokio::test]
    async fn test_image_to_vector() {
        let api = VectorApi::new(BASE_URL.to_string());
        let base64_image = "";
        let result = api.image_to_vector(vec![base64_image.into()]).await;

        assert!(result.is_ok(), "Image to vector request failed");
        let vectors = result.unwrap();
        assert!(!vectors.is_empty(), "Returned vector is empty");
        assert!(!vectors[0].is_empty(), "Returned vector has no elements");
    }

    #[tokio::test]
    async fn test_text_to_vector() {
        let api = VectorApi::new(BASE_URL.to_string());

        let text = "Hello, world!".to_string();

        let result = api.text_to_vector(vec![text]).await;

        assert!(result.is_ok(), "Text to vector request failed");
        let vectors = result.unwrap();
        assert!(!vectors.is_empty(), "Returned vector is empty");
        assert!(!vectors[0].is_empty(), "Returned vector has no elements");
    }
}
