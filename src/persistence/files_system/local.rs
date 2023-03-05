use std::path::Path;

use anyhow::anyhow;
use async_trait::async_trait;
use chicon::{OsFileSystem, FileSystem, OsFile};

use super::{file_type::FileType, dataset::Dataset};

pub struct LocalDataset {
    local_client: OsFileSystem,
    path: String,
}

#[async_trait]
impl Dataset for LocalDataset {
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

impl LocalDataset {
    pub async fn new<D: Dataset>(source_path: &str, file_type: &FileType) -> anyhow::Result<D> {
        match file_type {
            FileType::JSON => LocalDataset::load_json(source_path).await,
            FileType::CSV => LocalDataset::load_csv(source_path).await,
            FileType::PARQUET => LocalDataset::load_parquet(source_path).await
        }
    }
    
}

async fn open_local_file(source_path: &str, local_client: &OsFileSystem) -> anyhow::Result<OsFile> {
    let path = Path::new(source_path);
    let file = local_client.open_file(path)
        .map_err(|error| anyhow!(error))?;

    log::info!("Opening file {filename}", filename=source_path);

    Ok(file)
}