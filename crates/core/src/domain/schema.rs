use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::naming::NamingConvention;
use super::{Edge, Model, ObjectTypeDefinition};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub datasource: Datasource,
    #[serde(default)]
    pub naming: NamingConvention,
    pub generators: Vec<Generator>,
    #[serde(default)]
    pub object_types: Vec<ObjectTypeDefinition>,
    pub models: Vec<Model>,
    pub edges: Vec<Edge>,
}

impl DatabaseSchema {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            datasource: Datasource {
                provider: String::new(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: Vec::new(),
            models: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Datasource {
    pub provider: String,
    pub url: Option<String>,
    pub namespace: Option<String>,
    pub database: Option<String>,
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Generator {
    pub provider: String,
    pub output: String,
    pub extra: BTreeMap<String, String>,
}

/// Keep connection and DSL-only blocks from disk; replace models and edges from introspection.
#[must_use]
pub fn merge_pulled_schema(preserve: &DatabaseSchema, pulled: &DatabaseSchema) -> DatabaseSchema {
    DatabaseSchema {
        datasource: preserve.datasource.clone(),
        naming: preserve.naming.clone(),
        generators: preserve.generators.clone(),
        object_types: preserve.object_types.clone(),
        models: pulled.models.clone(),
        edges: pulled.edges.clone(),
    }
}
