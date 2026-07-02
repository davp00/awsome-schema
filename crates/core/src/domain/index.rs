use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::naming::{NamingConvention, mapped_name};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    pub name: Option<String>,
    pub fields: Vec<String>,
    pub unique: bool,
    pub fulltext: bool,
    pub vector: bool,
}

impl Index {
    #[must_use]
    pub fn resolved_name(&self, table: &str, naming: &NamingConvention) -> String {
        self.name.clone().unwrap_or_else(|| {
            let fields = self
                .fields
                .iter()
                .map(|field| mapped_name(field, &BTreeMap::new(), naming.fields))
                .collect::<Vec<_>>()
                .join("_");
            format!("{table}_{fields}_idx")
        })
    }
}
