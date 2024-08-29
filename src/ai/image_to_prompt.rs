use crate::ai::ResponseData;
use crate::constant::OLLAMA_ENDPOINT;
use base64::engine::general_purpose;
use base64::Engine;
use image::ImageReader;
use reqwest::Client;
use serde::Serialize;
use std::env;
use std::io::Cursor;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Serialize)]
struct RequestData {
    model: String,
    prompt: String,
    stream: bool,
    images: Vec<String>,
}

pub async fn image_to_prompt(image: impl AsRef<[u8]>) -> anyhow::Result<String> {
    let mut reader = ImageReader::new(Cursor::new(image)).with_guessed_format()?;
    let mut png_data: Vec<u8> = Vec::new();
    {
        let image = reader.decode()?;
        image.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png)?;
    }

    let client = Client::new();
    let request_data = RequestData {
        model: "llava".to_string(),
        prompt: "Use ten keywords to describe the content of the image, separating keywords with semicolon".to_string(),
        stream: false,
        images: vec![general_purpose::STANDARD.encode(&png_data)],
    };

    let response = client
        .post(format!("{}/api/generate", env::var(OLLAMA_ENDPOINT)?))
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

pub async fn read_image_from_path(
    image_path: impl AsRef<std::path::Path>,
) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(image_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

mod test {
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_image_to_prompt() {
        dotenv().ok();
        let image1 = crate::ai::image_to_prompt::read_image_from_path("test/image.png")
            .await
            .unwrap();
        let image2 = crate::ai::image_to_prompt::read_image_from_path("test/img2.jpeg")
            .await
            .unwrap();
        let image3 = crate::ai::image_to_prompt::read_image_from_path("test/thumbnail.png")
            .await
            .unwrap();

        println!(
            "image1 to prompt: {}",
            super::image_to_prompt(image1).await.unwrap()
        );
        println!(
            "image2 to prompt: {}",
            super::image_to_prompt(image2).await.unwrap()
        );
        println!(
            "image3 to prompt: {}",
            super::image_to_prompt(image3).await.unwrap()
        );
    }
}
