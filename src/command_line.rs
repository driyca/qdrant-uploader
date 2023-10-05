use clap::Parser;
use crate::persistence::{files_system::FileType, vector_field_name::FieldName};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct CommandLine {
    /// Source path
    #[clap(long, short, env = "SOURCE_PATH")]
    pub source_path: String,

    /// Source file type
    #[clap(long, default_value = "json", env = "SOURCE_FILE_TYPE")]
    pub source_file_type: FileType,

    /// Database connection String
    #[clap(long, env = "CONNECTION_STRING")]
    pub connection_string: String,

    /// Database collection
    #[clap(long, env = "DATABASE_COLLECTION")]
    pub database_collection: String,

    /// Field to be used as Qdrant point id
    #[clap(long)]
    pub id_field_name: Option<String>,

    /// Names of the fields to be loaded as vectors
    #[clap(long)]
    #[arg(num_args(1..))]
    pub vector_field_name: Vec<String>,

    /// If true, a non named vector is upload, but it is possible only if just one vector field name is provided
    #[clap(long, default_value="false")]
    pub upload_non_named_vector: bool,

    /// Names of the fields to be loaded as payload or the name 
    #[clap(long)]
    #[arg(num_args(0..))]
    pub payload_field: Option<Vec<String>>,

    /// If a single payload field is provided and it is an object, it will be uploaded as the payload value
    #[clap(long, default_value="false")]
    pub upload_whole_field_as_payload: bool,

    /// The Qdrant database write chunk size
    #[clap(long, default_value="256")]
    pub chunk_size: usize,

    /// Database collection
    #[clap(long, env = "BATCH_SIZE")]
    pub batch_size: u32,

    /// The S3 endpoint to connect and save file
    #[clap(long, env = "S3_ENDPOINT")]
    pub s3_endpoint: Option<String>,

    /// S3 Access key
    #[clap(long, env = "S3_ACCESS_KEY")]
    pub s3_access_key: Option<String>,

    /// S3 Secret Access key
    #[clap(long, env = "S3_SECRET_ACCESS_KEY")]
    pub s3_secret_access_key: Option<String>,

    /// S3 Region to connect
    #[clap(long, default_value="minio", env = "S3_REGION")]
    pub s3_region: Option<String>,
}


impl CommandLine {
    pub fn load_payload_field(&self) -> anyhow::Result<Option<FieldName>> {
        if self.payload_field.is_none() {
            Ok(None)
        } else {
            let payload_fields = self.payload_field.clone().unwrap();

            if payload_fields.len() > 1 && self.upload_whole_field_as_payload {
                anyhow::bail!("When using --upload-whole-field-as-payload=true, at most one value must be provided for --payload-field");
            } else {
                if self.upload_whole_field_as_payload {
                    let field_name = payload_fields.first().unwrap().to_owned();
                    Ok(Some(FieldName::Single(field_name)))
                } else {
                    Ok(Some(FieldName::Named(payload_fields)))
                }
            }
        }
    }

    pub fn load_vector_field_name(&self)  -> anyhow::Result<FieldName> {
        if self.vector_field_name.len() > 1 && self.upload_non_named_vector {
            anyhow::bail!("When using --updload-non-named-vector=true, at most one value must be provided for --vector-field-name");
        } else {
            if self.upload_non_named_vector {
                let field_name = self.vector_field_name.first().unwrap().to_owned();
                Ok(FieldName::Single(field_name))
            } else {
                Ok(FieldName::Named(self.vector_field_name.clone()))
            }
        }
    }
}