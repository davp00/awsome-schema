use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::naming::{NamingConvention, mapped_name};

/// Distance metric for HNSW vector indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VectorDist {
    #[default]
    Euclidean,
    Cosine,
    Manhattan,
}

impl VectorDist {
    #[must_use]
    pub fn as_dsl(self) -> &'static str {
        match self {
            Self::Euclidean => "Euclidean",
            Self::Cosine => "Cosine",
            Self::Manhattan => "Manhattan",
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "euclidean" => Some(Self::Euclidean),
            "cosine" => Some(Self::Cosine),
            "manhattan" => Some(Self::Manhattan),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_surreal(self) -> &'static str {
        match self {
            Self::Euclidean => "EUCLIDEAN",
            Self::Cosine => "COSINE",
            Self::Manhattan => "MANHATTAN",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    pub name: Option<String>,
    pub fields: Vec<String>,
    pub unique: bool,
    pub fulltext: bool,
    pub fulltext_analyzer: Option<String>,
    pub vector: bool,
    pub vector_dimension: Option<u32>,
    pub vector_dist: Option<VectorDist>,
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

#[cfg(test)]
mod tests {
    use super::VectorDist;

    #[test]
    fn vector_dist_parses_case_insensitively() {
        assert_eq!(VectorDist::parse("Cosine"), Some(VectorDist::Cosine));
        assert_eq!(VectorDist::parse("COSINE"), Some(VectorDist::Cosine));
        assert_eq!(VectorDist::parse("euclidean"), Some(VectorDist::Euclidean));
        assert_eq!(VectorDist::parse("manhattan"), Some(VectorDist::Manhattan));
        assert_eq!(VectorDist::parse("other"), None);
    }

    #[test]
    fn vector_dist_formats_dsl_and_surreal() {
        assert_eq!(VectorDist::Cosine.as_dsl(), "Cosine");
        assert_eq!(VectorDist::Cosine.as_surreal(), "COSINE");
        assert_eq!(VectorDist::Euclidean.as_dsl(), "Euclidean");
        assert_eq!(VectorDist::Euclidean.as_surreal(), "EUCLIDEAN");
        assert_eq!(VectorDist::Manhattan.as_dsl(), "Manhattan");
        assert_eq!(VectorDist::Manhattan.as_surreal(), "MANHATTAN");
    }
}
