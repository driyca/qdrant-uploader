use mongodb::{Collection, Client, Database, options::ClientOptions};

const APPLICATION_NAME: &str = "mongodb-uploader";

pub struct DatabaseClient {
    database_name: String,
    collection_name: String,
    collection: Collection<serde_json::Value>
}

impl DatabaseClient {

    pub async fn new(connection_string: &str, database_name: &str,  collection_name: &str) -> anyhow::Result<DatabaseClient> {
        let database = make_database(connection_string, database_name).await?;
        let collection = make_mongodb_collection(&database, collection_name);

        let database_client =
            DatabaseClient {
                database_name: database_name.to_string(),
                collection_name: collection_name.to_string(),
                collection: collection
            };

        Ok(database_client)
    }

    pub async fn insert_batch(&self, batch: Vec<serde_json::Value>) -> anyhow::Result<()> {
        self.collection.insert_many(batch, None).await?;
        Ok(())
    }
}

async fn make_database(connection_string: &str, database_name: &str) -> anyhow::Result<Database> {
    let mut client_options = ClientOptions::parse(connection_string).await?;
    client_options.app_name = Some(APPLICATION_NAME.to_owned());

    let client = Client::with_options(client_options)?;
    let database = client.database(database_name);

    Ok(database)
}

fn make_mongodb_collection(database: &Database, collection_name: &str) -> Collection<serde_json::Value> {
    let collection = database.collection::<serde_json::Value>(collection_name);
    collection
}