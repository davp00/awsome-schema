use serde::{Deserialize, Serialize};

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
    pub fn resolved_name(&self, table: &str) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("{table}_{}_idx", self.fields.join("_").to_lowercase()))
    }
}
