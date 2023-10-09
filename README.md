# qdrant-uploader

A system to upload JSON or CSV files to Qdrant from local file os s3-like object storage.

## Usage

```
$> mongodb-uploader --help

Usage: qdrant-uploader [OPTIONS] --source-path <SOURCE_PATH> --connection-string <CONNECTION_STRING> --database-collection <DATABASE_COLLECTION> --batch-size <BATCH_SIZE>

Options:
  -s, --source-path <SOURCE_PATH>
          Source path [env: SOURCE_PATH=]
      --source-file-type <SOURCE_FILE_TYPE>
          Source file type [env: SOURCE_FILE_TYPE=] [default: json] [possible values: json, csv]
      --connection-string <CONNECTION_STRING>
          QDrant connection String [env: CONNECTION_STRING=]
      --api-key <API_KEY>
          Qdrant connection String [env: DATABASE_API_KEY=]
      --database-collection <DATABASE_COLLECTION>
          Qdrant collection [env: DATABASE_COLLECTION=]
      --id-field-name <ID_FIELD_NAME>
          Field to be used as Qdrant point id
      --vector-field-name <VECTOR_FIELD_NAME>...
          Names of the fields to be loaded as vectors
      --upload-non-named-vector
          If true, a non named vector is upload, but it is possible only if just one vector field name is provided
      --payload-field [<PAYLOAD_FIELD>...]
          Names of the fields to be loaded as payload or the name
      --upload-whole-field-as-payload
          If a single payload field is provided and it is an object, it will be uploaded as the payload value
      --chunk-size <CHUNK_SIZE>
          The Qdrant database write chunk size [default: 256]
      --batch-size <BATCH_SIZE>
          Database collection [env: BATCH_SIZE=]
      --s3-endpoint <S3_ENDPOINT>
          The S3 endpoint to connect and save file [env: S3_ENDPOINT=]
      --s3-access-key <S3_ACCESS_KEY>
          S3 Access key [env: S3_ACCESS_KEY=]
      --s3-secret-access-key <S3_SECRET_ACCESS_KEY>
          S3 Secret Access key [env: S3_SECRET_ACCESS_KEY=]
      --s3-region <S3_REGION>
          S3 Region to connect [env: S3_REGION=] [default: minio]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Docker image

A docker image is available at `docker.io/andreclaudino/qdrant-uploader`.

Current available tags are:

* latest
* 0.1.0
* 0.2.0

