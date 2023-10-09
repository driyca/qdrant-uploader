use clap::Parser;
use persistence::files_system::Dataset;
use processors::run_transference;
use crate::{command_line::CommandLine, persistence::DatabaseClient};

mod persistence;
mod command_line;
mod processors;


#[tokio::main(flavor="current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let arguments = CommandLine::parse();
    let payload_fields = arguments.load_payload_field()?;
    let vector_fields = arguments.load_vector_field_name()?;
    
    let database_client =
        DatabaseClient::new(&arguments.connection_string, &arguments.api_key, &arguments.database_collection,
            arguments.id_field_name.clone(), vector_fields,
            payload_fields, arguments.chunk_size,
        ).await?;

    let dataset = Dataset::load(&arguments.source_path, &arguments.source_file_type, arguments.s3_access_key, arguments.s3_secret_access_key,
                                        arguments.s3_region, arguments.s3_endpoint).await?;

    run_transference(&database_client, &dataset, arguments.batch_size).await?;

    Ok(())
}
