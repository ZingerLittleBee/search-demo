use crate::constant::{S3_ACCESS_KEY, S3_BUCKET, S3_ENDPOINT, S3_SECRET_KEY};
use minio::s3::args::{BucketExistsArgs, MakeBucketArgs};
use minio::s3::builders::ObjectContent;
use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use std::env;
use tracing::info;

pub struct S3 {
    client: Client,
}

impl S3 {
    pub async fn new() -> Self {
        Self {
            client: Self::init_minio().await.expect("init s3 error"),
        }
    }

    async fn init_minio() -> anyhow::Result<Client> {
        let base_url = env::var(S3_ENDPOINT)?.parse::<BaseUrl>()?;
        info!("Trying to connect to MinIO at: `{:?}`", base_url);

        let static_provider = StaticProvider::new(
            env::var(S3_ACCESS_KEY)?.as_str(),
            env::var(S3_SECRET_KEY)?.as_str(),
            None,
        );

        let client = ClientBuilder::new(base_url.clone())
            .provider(Some(Box::new(static_provider)))
            .build()?;

        let bucket_name = env::var(S3_BUCKET)?;

        let exists: bool = client
            .bucket_exists(&BucketExistsArgs::new(&bucket_name)?)
            .await?;

        if !exists {
            client
                .make_bucket(&MakeBucketArgs::new(&bucket_name)?)
                .await?;
        }
        Ok(client)
    }

    pub async fn upload_image(&self, file_name: &str, content: Vec<u8>) -> anyhow::Result<()> {
        let content = ObjectContent::from(content);
        self.client
            .put_object_content(env::var(S3_BUCKET)?.as_str(), file_name, content)
            .send()
            .await?;

        info!(
            "file `{}` is successfully uploaded to bucket `{}`.",
            file_name,
            env::var(S3_BUCKET)?
        );
        Ok(())
    }
}
