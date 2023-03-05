use std::path::Path;

use anyhow::anyhow;
use async_trait::async_trait;
use chicon::{S3File, FileSystem};

use super::{file_type::FileType, dataset::Dataset};

pub struct S3Dataset {
    s3_client: chicon::S3FileSystem,
    bucket: String,
    key: String,
}

#[async_trait]
impl Dataset for S3Dataset {
    async fn next_line(&self) -> anyhow::Result<serde_json::Value> {
        todo!();
    }

    async fn load_json<D: Dataset>(source_path: &str) -> anyhow::Result<D> {
        todo!()
    }

    async fn load_csv<D: Dataset>(source_path: &str) -> anyhow::Result<D> {
        todo!()
    }

    async fn load_parquet<D: Dataset>(source_path: &str) -> anyhow::Result<D> {
        todo!()
    }

    async fn next_feed_item(&mut self) -> anyhow::Result<Option<serde_json::Value>> {
        todo!()
    }
}

impl S3Dataset {
    pub async fn new<D: Dataset>(source_path: &str, file_type: &FileType) -> anyhow::Result<D> {
        match file_type {
            FileType::JSON => S3Dataset::load_json(source_path).await,
            FileType::CSV => S3Dataset::load_csv(source_path).await,
            FileType::PARQUET => S3Dataset::load_parquet(source_path).await
        }
    }

}

async fn open_s3_file(bucket: &str, key: &str, s3_client: &S3Dataset) -> anyhow::Result<S3File> {
    let path = Path::new(key);
    let file =
        s3_client
            .s3_client.open_file(path)
            .map_err(|error| anyhow!(error))?;

    log::info!("Opening file s3://{bucket}/{filename}", bucket=bucket, filename=key);

    Ok(file)
}