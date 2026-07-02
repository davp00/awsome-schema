use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relation {
    pub name: String,
    pub from_model: String,
    pub to_model: String,
    pub from_field: String,
    pub to_field: String,
}
