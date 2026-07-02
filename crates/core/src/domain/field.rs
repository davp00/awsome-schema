use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
    pub unique: bool,
    pub is_id: bool,
    /// Applied on INSERT when no value is provided (`DEFAULT` clause).
    pub default_value: Option<String>,
    /// Also applied on UPDATE when the value is empty (`DEFAULT ALWAYS` clause).
    pub default_always: bool,
    /// Recomputed on every CREATE and UPDATE (`VALUE` clause, `@value` / `@updated`).
    pub value_expression: Option<String>,
    /// Prevents manual updates (`READONLY` clause, typically with `@value`).
    pub readonly: bool,
    pub link_target: Option<String>,
    pub relation_name: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

impl Field {
    /// Whether this field uses a computed `VALUE` clause instead of a static `DEFAULT`.
    #[must_use]
    pub const fn has_value_expression(&self) -> bool {
        self.value_expression.is_some()
    }
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
    pub fn surreal_type_name(&self, optional: bool) -> String {
        let inner = self.base_surreal_type_name();
        if optional { format!("option<{inner}>") } else { inner }
    }

    #[must_use]
    pub fn base_surreal_type_name(&self) -> String {
        match self {
            Self::String => "string".to_owned(),
            Self::Int => "int".to_owned(),
            Self::Float => "float".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Datetime => "datetime".to_owned(),
            Self::Object => "object".to_owned(),
            Self::Array(inner) => format!("array<{}>", inner.base_surreal_type_name()),
            Self::RecordId(target) => format!("record<{target}>"),
            Self::Model(name) => format!("record<{name}>"),
            Self::Custom(value) => value.clone(),
        }
    }
}
