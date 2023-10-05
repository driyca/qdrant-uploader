use std::collections::HashMap;
use std::str::FromStr;

use qdrant_client::qdrant::NamedVectors;
use qdrant_client::qdrant::PointId;
use qdrant_client::qdrant::PointStruct;
use qdrant_client::qdrant::Value as QdrantValue;
use qdrant_client::qdrant::Vector;
use qdrant_client::qdrant::Vectors;
use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::vectors::VectorsOptions;

use crate::persistence::vector_field_name::FieldName;

pub fn batch_to_points(batch: Vec<serde_json::Value>, id_field_name: Option<String>, vector_field: &FieldName, payload_field: &Option<FieldName>) -> anyhow::Result<Vec<PointStruct>> {
    let points = 
        batch.iter()
            .map(|value| {
                value_to_point(value, &id_field_name, vector_field, payload_field)
            })
            .collect();
    Ok(points)
}


fn value_to_point(value: &serde_json::Value, maybe_id_field_name: &Option<String>, vector_field_names: &FieldName, maybe_payload_field: &Option<FieldName>) -> PointStruct {
    let id = extract_point_id(maybe_id_field_name, value);
    let payload = extract_payload(maybe_payload_field, value);
    let vectors = extract_vectors(vector_field_names, value);

    PointStruct { id, payload, vectors }
}


/// TODO: Essa função precisa ser refatorada
fn extract_point_id(maybe_id_field_name: &Option<String>, source_value: &serde_json::Value) -> Option<PointId> {
    let id_options =
        if let Some(id_field_name) = maybe_id_field_name {
            if let Some(value) = source_value.get(id_field_name) {
                if let Some(id_value) = value.as_u64() {
                    Some(PointIdOptions::Num(id_value))
                } else if let Some(id_value) = value.as_str() {
                    if let Ok(_) = uuid::Uuid::from_str(id_value) {
                        Some(PointIdOptions::Uuid(id_value.to_owned()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
    
    let point_id = id_options.map(|child|
        PointId {
            point_id_options: Some(child)
        }
    );

    point_id
}

fn extract_payload(maybe_payload_fields: &Option<FieldName>, value: &serde_json::Value) -> HashMap<String, QdrantValue>{
    match maybe_payload_fields {
        Some(FieldName::Single(field_name)) => {
            let maybe_field_content = value.get(field_name);
            if maybe_field_content.is_none() {
                return HashMap::new()
            } else {
                extract_payload_from_single_field(maybe_field_content)
            }
        },
        Some(FieldName::Named(field_names)) => {
            extract_payload_from_multiple_fields(field_names, value)
        },
        None => {
            HashMap::new()
        }
    }
}

fn extract_payload_from_multiple_fields(field_names: &Vec<String>, value: &serde_json::Value) -> HashMap<String, QdrantValue> {
    field_names.iter().map(|field_name|{
        let field_value = value.get(field_name).map(|value| QdrantValue::from(value.to_owned())).unwrap_or_default();
        (field_name.to_owned(), field_value)
        }).collect()
}

fn extract_payload_from_single_field(maybe_field_content: Option<&serde_json::Value>) -> HashMap<String, QdrantValue> {
    let field_content = maybe_field_content.unwrap();
    let maybe_object = field_content.as_object();
    match maybe_object {
        None => HashMap::new(),
        Some(object) => {
            object.iter().map(|(key, value)| {
                let qdrant_value = QdrantValue::from(value.to_owned());
                (key.to_owned(), qdrant_value)
            }).collect()
        }
    }
}

fn extract_vectors(vector_field_names: &FieldName, value: &serde_json::Value) -> Option<Vectors> {
    match vector_field_names {
        FieldName::Named(field_names) => {
            extract_named_vectors(field_names, value)
        },
        FieldName::Single(field_name) => {
            extract_single_vector(value, field_name)
        }
    }
}

fn extract_named_vectors(field_names: &Vec<String>, value: &serde_json::Value) -> Option<Vectors> {
    let vectors_with_names = 
        extract_vectors_with_names(field_names, value);
            
    let named_vectors = NamedVectors {vectors: vectors_with_names};
    let vector_options = VectorsOptions::Vectors(named_vectors);
    let vectors = Vectors { vectors_options: Some(vector_options) };
    Some(vectors)
}

fn extract_vectors_with_names(field_names: &Vec<String>, value: &serde_json::Value) -> HashMap<String, Vector> {
    field_names.iter().filter_map(|field_name|{
        extract_qdrant_vector(value, field_name)
            .map(|qdrant_vector|{
                (field_name.to_owned(), qdrant_vector)
            })
    }).collect()
}

fn extract_single_vector(value: &serde_json::Value, field_name: &String) -> Option<Vectors> {
    let maybe_qdrant_vector = extract_qdrant_vector(value, field_name);
    let maybe_qdrant_vector_option = maybe_qdrant_vector.map(|qdrant_vector| VectorsOptions::Vector(qdrant_vector));
    let vectors = Vectors {vectors_options: maybe_qdrant_vector_option};
    Some(vectors)
}

fn extract_qdrant_vector(value: &serde_json::Value, field_name: &String) -> Option<Vector> {
    let maybe_field_value = value.get(field_name);
    if maybe_field_value.is_none() {
        None
    } else {
        let field_value = maybe_field_value.unwrap();
    
        let maybe_vector = field_value.as_array();
        if maybe_vector.is_none() {
            None
        } else {
            let vector = maybe_vector.unwrap();
            let vector_data = vector.iter().map(|coordinate| coordinate.as_f64().unwrap_or_default() as f32).collect();
            let qdrant_vector = Vector {
                data: vector_data
            };
            Some(qdrant_vector)
        }
    }
}