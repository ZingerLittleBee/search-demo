use reqwest::Client;
use url::Url;

pub async fn load_image_from_url(url: Url) -> anyhow::Result<Vec<u8>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes: Vec<u8> = response.bytes().await?.to_vec();
    Ok(bytes)
}
