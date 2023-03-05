use clap::Parser;
use persistence::files_system::load_dataset;
use crate::{command_line::CommandLine, persistence::DatabaseClient};

mod persistence;
mod command_line;


#[tokio::main(flavor="current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let arguments = CommandLine::parse();
    
    let database_client =
        DatabaseClient::new(&arguments.connection_string, &arguments.database_name, &arguments.database_collection).await?;

    let dataset = load_dataset(&arguments.source_path, &arguments.source_file_type).await?;

    Ok(())
}
