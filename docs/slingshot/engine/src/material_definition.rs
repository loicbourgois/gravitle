use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ValueOrReference {
    Value(f32),
    Reference(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct MaterialDefinition {
    pub density: ValueOrReference,
    pub color: String,
    pub storage: Option<HashMap<String, u32>>,
    pub production: Option<HashMap<String, u32>>,
}
