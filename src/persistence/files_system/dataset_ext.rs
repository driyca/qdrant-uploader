use async_trait::async_trait;

#[async_trait]
pub trait DatasetExt {
    type DatasetType: DatasetExt;

    async fn next_line(&self) -> anyhow::Result<Option<serde_json::Value>>;

    async fn next_batch(&self, batch_size: u32) -> anyhow::Result<Option<Vec<serde_json::Value>>> {
        let mut batch = Vec::new();

        for batch_row in 0..batch_size {
            let next_value = self.next_line().await?;
            
            if let Some(value) = next_value {
                batch.push(value);
            } else {
                log::info!("Total of {batch_row} elements loaded in batch (maximum is {batch_size})");
                break;
            }
        }
        
        if batch.is_empty() {
            Ok(None)
        } else {
            Ok(Some(batch))
        }
    }

}