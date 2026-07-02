use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
    pub unique: bool,
    pub is_id: bool,
    pub default_value: Option<String>,
    pub link_target: Option<String>,
    pub relation_name: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
    Datetime,
    Object,
    Array(Box<FieldType>),
    RecordId(String),
    Model(String),
    Custom(String),
}

impl FieldType {
    #[must_use]
    pub fn surreal_type_name(&self) -> String {
        match self {
            Self::String => "string".to_owned(),
            Self::Int => "int".to_owned(),
            Self::Float => "float".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Datetime => "datetime".to_owned(),
            Self::Object => "object".to_owned(),
            Self::Array(inner) => format!("array<{}>", inner.surreal_type_name()),
            Self::RecordId(target) => format!("record<{target}>"),
            Self::Model(name) => format!("record<{name}>"),
            Self::Custom(value) => value.clone(),
        }
    }
}
