use crate::persistence::{DatabaseClient, files_system::Dataset};
use crate::persistence::files_system::DatasetExt;

pub async fn run_transference(database_client: &DatabaseClient, dataset: &Dataset,
                              batch_size: u32) -> anyhow::Result<()> {
    
    while let Some(batch) = dataset.next_batch(batch_size).await? {
        database_client.insert_batch(batch).await?;
    }

    Ok(())
}