use std::{io::{BufReader, BufRead, Lines, Cursor}};

use async_trait::async_trait;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::{Credentials, Region};
use bytes::Bytes;
use tokio::{sync::RwLock};
use url::Url;

use super::{file_type::FileType, dataset_ext::DatasetExt};

pub struct S3Dataset {
    lines: RwLock<Lines<BufReader<Cursor<Bytes>>>>,
    file_type: FileType,
    csv_header: Option<String>,
}

impl S3Dataset {
    pub async fn new(source_path: &str, file_type: &FileType,
                    access_key: &str, secret_key: &str, region_name: &str, endpoint_url: &str) -> anyhow::Result<Self> {
        
        let (bucket, key) = split_bucket_and_key(source_path)?;
        let s3_config = make_s3_config(access_key, secret_key, region_name, endpoint_url);
        let s3_client = make_s3_client(s3_config)?;

        match file_type {
            FileType::JSON => S3Dataset::load_json(&bucket, &key, &s3_client).await,
            FileType::CSV => S3Dataset::load_csv(source_path, &key, &s3_client).await,
        }
    }

    async fn load_json(bucket_name: &str, key: &str, s3_client: &aws_sdk_s3::Client) -> anyhow::Result<Self> {
        let lines = open_s3_file(bucket_name, key, s3_client).await?;
        let lines_lock = RwLock::new(lines);

        let dataset = Self {
            lines: lines_lock,
            file_type: FileType::JSON,
            csv_header: None
        };

        Ok(dataset)
    }

    async fn load_csv(bucket_name: &str, key: &str, s3_client: &aws_sdk_s3::Client) -> anyhow::Result<Self> {
        let mut lines = open_s3_file(bucket_name, key, s3_client).await?;
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

fn make_s3_client(s3_config: aws_sdk_s3::Config) -> anyhow::Result<aws_sdk_s3::Client> { 
    let client = aws_sdk_s3::Client::from_conf(s3_config);

    Ok(client)
}

fn make_s3_config(access_key: &str, secret_key: &str, _region_name: &str, endpoint_url: &str) -> aws_sdk_s3::Config {
    let credentials = Credentials::new(
        access_key,
        secret_key,
        None,
        None,
        "InternalProvider"
    );
    let credential_providers = SharedCredentialsProvider::new(credentials);
    
    let config = make_config(endpoint_url, credential_providers);

    config
}

fn make_config(endpoint_url: &str, credential_providers: SharedCredentialsProvider) -> aws_sdk_s3::Config {
    let config_builder = aws_sdk_s3::Config::builder();
    
    let config =
        config_builder
            .force_path_style(true)
            .endpoint_url(endpoint_url)
            .set_credentials_provider(Some(credential_providers))
            .build();

    config
}


fn split_bucket_and_key(source_path: &str) -> anyhow::Result<(String, String)> {
    let url = Url::parse(source_path)?;
    if let Some(bucket) = url.host_str() {
        let key = url.path().strip_prefix("/").unwrap_or("");
        Ok((bucket.to_owned(), key.to_owned()))
    } else {
        anyhow::bail!("Invalid source-path: {source_path}")
    }
    
}

async fn open_s3_file(bucket: &str, key: &str, s3_client: &aws_sdk_s3::Client) -> anyhow::Result<Lines<BufReader<Cursor<Bytes>>>> {   
    let request =
        s3_client.get_object().bucket(bucket).key(key);
    
    let response = request.send().await?;
    let response_bytes = response.body.collect().await?.into_bytes();

    let cursor = Cursor::new(response_bytes);
    let reader = BufReader::new(cursor).lines();

    log::info!("Opening file s3://{bucket}/{filename}", bucket=bucket, filename=key);

    Ok(reader)
}