use async_trait::async_trait;

#[async_trait]
pub trait Dataset {
    async fn next_line(&self) -> anyhow::Result<serde_json::Value>;

    async fn next_batch(&self, batch_size: u32) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut batch = Vec::new();

        for batch_row in 0..batch_size {
            let value = self.next_line().await?;
            batch.push(value);
        }

        Ok(batch)
    }

    async fn load_json<D: Dataset>(source_path: &str) -> anyhow::Result<D>;

    async fn load_csv<D: Dataset>(source_path: &str) -> anyhow::Result<D>;

    async fn load_parquet<D: Dataset>(source_path: &str) -> anyhow::Result<D>;

    async fn next_feed_item(&mut self) -> anyhow::Result<Option<serde_json::Value>>;
}