use std::sync::Arc;

use qdrant_client::{prelude::QdrantClient, qdrant::WriteOrdering};

use crate::persistence::vector_field_name::FieldName;

use super::batch_processor::batch_to_points;

pub struct DatabaseClient {
    client: Arc<QdrantClient>,
    collection_name: String,

    id_field: Option<String>, 
    vector_field: FieldName,
    payload_field: Option<FieldName>,
    
    write_ordering: Option<WriteOrdering>,
    chunk_size: usize,
}

impl DatabaseClient {

    pub async fn new(connection_string: &str, api_key: &Option<String>, collection_name: &str, id_field_name: Option<String>, vector_field: FieldName,
            payload_field: Option<FieldName>, chunk_size: usize) -> anyhow::Result<DatabaseClient> {
                
        let client = QdrantClient::from_url(connection_string).with_api_key(api_key.to_owned()).build()?;
        let arc_client = Arc::new(client);
        
        let database_client = DatabaseClient{
            client: arc_client,
            collection_name: collection_name.to_owned(),
            vector_field,
            id_field: id_field_name.to_owned(),
            payload_field,
            write_ordering: None,
            chunk_size
        };

        Ok(database_client)
    }

    pub async fn insert_batch(&self, batch: Vec<serde_json::Value>) -> anyhow::Result<()> {
        
        let points = batch_to_points(batch, self.id_field.clone(), &self.vector_field, &self.payload_field)?;
        self.client.upsert_points_batch_blocking(&self.collection_name, points, self.write_ordering.clone(), self.chunk_size).await?;
        Ok(())
    }
}