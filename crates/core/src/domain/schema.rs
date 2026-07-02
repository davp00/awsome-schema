use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::naming::NamingConvention;
use super::{Edge, Model};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub datasource: Datasource,
    #[serde(default)]
    pub naming: NamingConvention,
    pub generators: Vec<Generator>,
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
