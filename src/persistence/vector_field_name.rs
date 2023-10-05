#[derive(Clone)]
pub enum FieldName {
    Single(String),
    Named(Vec<String>)
}