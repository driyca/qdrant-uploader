use self::{dataset::Dataset, s3::S3Dataset, local::LocalDataset};

mod local;
mod s3;
mod file_type;
mod dataset;

pub use file_type::FileType;


pub async fn load_dataset<D: Dataset>(source_path: &str, file_type: &FileType) -> anyhow::Result<D> {
    let dataset: D = 
        if source_path.starts_with("s3://") {
            S3Dataset::new(source_path, file_type).await?
        } else {
            LocalDataset::new(source_path, file_type).await?
        };

    Ok(dataset)
}