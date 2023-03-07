use clap::{Parser};

use crate::persistence::files_system::FileType;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct CommandLine {
    /// Source path
    #[clap(long, short, env = "SOURCE_PATH")]
    pub source_path: String,

    /// Source file type
    #[clap(long, default_value = "json", env = "SOURCE_FILE_PATH")]
    pub source_file_type: FileType,

    /// Database connection String
    #[clap(long, env = "CONNECTION_STRING")]
    pub connection_string: String,

    /// Database name
    #[clap(long, env = "DATABASE_NAME")]
    pub database_name: String,

    /// Database collection
    #[clap(long, env = "DATABASE_COLLECTION")]
    pub database_collection: String,

    /// Database collection
    #[clap(long, env = "BATCH_SIZE")]
    pub batch_size: u32,

    /// The S3 endpoint to connect and save file
    #[clap(long, default_value="http://minio.storage.svc", env = "S3_ENDPOINT")]
    pub s3_endpoint: String,

    /// S3 Region to connect (blank for minio)
    #[clap(long, default_value="", env = "S3_NEW_PATH_STYLE")]
    pub s3_region: String,

    /// S3 Access key
    #[clap(long, env = "S3_ACCESS_KEY")]
    pub s3_access_key: String,

    /// S3 Secret Access key
    #[clap(long, env = "S3_SECRET_ACCESS_KEY")]
    pub s3_secret_access_key: String,
}