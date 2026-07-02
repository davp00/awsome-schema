use serde::{Deserialize, Serialize};

use super::FieldType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectTypeField {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectTypeDefinition {
    pub name: String,
    pub flexible: bool,
    pub fields: Vec<ObjectTypeField>,
}
