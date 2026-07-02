use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Field, Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TableMode {
    Schemafull,
    Schemaless,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
    pub table_mode: TableMode,
    pub permissions: Option<String>,
    pub indexes: Vec<Index>,
    pub attributes: BTreeMap<String, String>,
}

impl Model {
    #[must_use]
    pub fn table_name(&self) -> String {
        self.attributes.get("map").cloned().unwrap_or_else(|| self.name.to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub name: String,
    pub in_model: String,
    pub out_model: String,
    pub fields: Vec<Field>,
    pub table_mode: TableMode,
    pub permissions: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

impl Edge {
    #[must_use]
    pub fn table_name(&self) -> String {
        self.attributes.get("map").cloned().unwrap_or_else(|| self.name.to_lowercase())
    }
}
