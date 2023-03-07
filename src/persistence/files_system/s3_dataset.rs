use std::{path::Path, io::{BufReader, BufRead, Lines}};

use anyhow::anyhow;
use async_trait::async_trait;
use chicon::{S3File, FileSystem, S3FileSystem};
use tokio::{sync::RwLock};
use url::Url;

use super::{file_type::FileType, dataset_ext::DatasetExt};

pub struct S3Dataset {
    lines: RwLock<Lines<BufReader<S3File>>>,
    file_type: FileType,
    csv_header: Option<String>,
}

impl S3Dataset {
    pub async fn new(source_path: &str, file_type: &FileType,
                    access_key: &str, secret_key: &str, region: &str, endpoint: &str) -> anyhow::Result<Self> {
        let (bucket, key) = split_bucket_and_key(source_path)?;
        let s3_file_system = make_s3_client(&bucket, access_key, secret_key, region, endpoint)?;

        match file_type {
            FileType::JSON => S3Dataset::load_json(&bucket, &key, &s3_file_system).await,
            FileType::CSV => S3Dataset::load_csv(source_path, &key, &s3_file_system).await,
        }
    }

    async fn load_json(bucket_name: &str, key: &str, s3_file_system: &S3FileSystem) -> anyhow::Result<Self> {
        let lines = open_s3_file(bucket_name, key, s3_file_system).await?;
        let lines_lock = RwLock::new(lines);

        let dataset = Self {
            lines: lines_lock,
            file_type: FileType::JSON,
            csv_header: None
        };

        Ok(dataset)
    }

    async fn load_csv(bucket_name: &str, key: &str, s3_file_system: &S3FileSystem) -> anyhow::Result<Self> {
        let mut lines = open_s3_file(bucket_name, key, s3_file_system).await?;
        let csv_header = lines.next().unwrap()?;
        let csv_header = Some(csv_header);

        let lines_lock = RwLock::new(lines);

        let database = S3Dataset {
            file_type: FileType::CSV,
            lines: lines_lock,
            csv_header
        };

        Ok(database)
    }

}

#[async_trait]
impl DatasetExt for S3Dataset {
    
    type DatasetType = Self;

    async fn next_line(&self) -> anyhow::Result<Option<serde_json::Value>> {
        let mut unlocked_lines = self.lines.write().await;
        
        if let Some(current_line) = unlocked_lines.next() {
            match self.file_type {
                FileType::JSON => {        
                    let value = serde_json::from_str(&current_line?)?;
                    Ok(value)
                },
                FileType::CSV => {
                    let csv_line_with_header = self.csv_header.clone().unwrap() + "\n" + &current_line?;
                    let mut csv_reader = csv::Reader::from_reader(csv_line_with_header.as_bytes());
                    let mut csv_iter = csv_reader.deserialize();

                    let value: serde_json::Value = csv_iter.next().unwrap()?;
                    Ok(Some(value))
                }
            }
        } else {
            Ok(None)
        }
    }
    
}

fn make_s3_client(bucket_name: &str, access_key: &str, secret_key: &str, region: &str, endpoint: &str) -> anyhow::Result<S3FileSystem> {
    let file_system = S3FileSystem::new(
        access_key.to_owned(),
        secret_key.to_owned(),
        bucket_name.to_owned(),
        region.to_owned(),
        endpoint.to_owned()
    );

    Ok(file_system)
}

fn split_bucket_and_key(source_path: &str) -> anyhow::Result<(String, String)> {
    let url = Url::parse(source_path)?;
    if let Some(bucket) = url.host_str() {
        let key = url.path();

        Ok((bucket.to_owned(), key.to_owned()))
    } else {
        anyhow::bail!("Invalid source -path: {source_path}")
    }
    
}

async fn open_s3_file(bucket: &str, key: &str, s3_file_system: &S3FileSystem) -> anyhow::Result<Lines<BufReader<S3File>>> {
    let path = Path::new(key);
    let file =
        s3_file_system.open_file(path)
            .map_err(|error| anyhow!(error))?;
    
    let reader = BufReader::new(file).lines();

    log::info!("Opening file s3://{bucket}/{filename}", bucket=bucket, filename=key);

    Ok(reader)
}