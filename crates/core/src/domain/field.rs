use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Referential action when a linked record is deleted (provider-neutral).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OnDeleteAction {
    #[default]
    Ignore,
    Unset,
    Cascade,
    Reject,
}

impl OnDeleteAction {
    #[must_use]
    pub fn as_dsl(self) -> &'static str {
        match self {
            Self::Ignore => "Ignore",
            Self::Unset => "Unset",
            Self::Cascade => "Cascade",
            Self::Reject => "Reject",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "ignore" => Some(Self::Ignore),
            "unset" => Some(Self::Unset),
            "cascade" => Some(Self::Cascade),
            "reject" => Some(Self::Reject),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_surreal(self) -> &'static str {
        match self {
            Self::Ignore => "IGNORE",
            Self::Unset => "UNSET",
            Self::Cascade => "CASCADE",
            Self::Reject => "REJECT",
        }
    }
}

/// Whether a `@link` field stores the foreign key or is a computed opposite side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStorage {
    Stored,
    Computed,
}

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
    /// `SurrealDB` `FLEXIBLE` object: schemaless nested fields on a schemafull table.
    #[serde(default)]
    pub flexible: bool,
    pub link_target: Option<String>,
    /// Prisma-style relation pair name shared by both sides of a `@link`.
    #[serde(default)]
    pub link_name: Option<String>,
    /// Delete policy on the stored side of a reference (default Ignore when stored).
    #[serde(default)]
    pub on_delete: Option<OnDeleteAction>,
    /// Resolved by normalize: stored FK vs computed opposite.
    #[serde(default)]
    pub link_storage: Option<LinkStorage>,
    /// On a computed side: DSL name of the stored field on the opposite model.
    #[serde(default)]
    pub link_opposite_field: Option<String>,
    pub relation_name: Option<String>,
    pub attributes: BTreeMap<String, String>,
}

impl Field {
    /// Whether this field uses a computed `VALUE` clause instead of a static `DEFAULT`.
    #[must_use]
    pub const fn has_value_expression(&self) -> bool {
        self.value_expression.is_some()
    }

    /// Whether this field is a record reference (`@link`).
    #[must_use]
    pub fn is_link(&self) -> bool {
        self.link_target.is_some()
    }

    /// Whether the link type is an array of records (many side).
    #[must_use]
    pub fn is_list_link(&self) -> bool {
        matches!(self.field_type, FieldType::Array(_))
    }

    #[must_use]
    pub fn is_computed_link(&self) -> bool {
        matches!(self.link_storage, Some(LinkStorage::Computed))
    }

    #[must_use]
    pub fn is_stored_link(&self) -> bool {
        matches!(self.link_storage, Some(LinkStorage::Stored))
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

    /// Model name for a link field type (`User` or `User[]`).
    #[must_use]
    pub fn link_model_name(&self) -> Option<&str> {
        match self {
            Self::Model(name) => Some(name.as_str()),
            Self::Array(inner) => match inner.as_ref() {
                Self::Model(name) => Some(name.as_str()),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_value_expression_when_set() {
        let field = Field {
            name: "updatedAt".to_owned(),
            field_type: FieldType::Datetime,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: Some("time::now()".to_owned()),
            readonly: false,
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };
        assert!(field.has_value_expression());
    }

    #[test]
    fn surreal_type_names_cover_all_variants() {
        assert_eq!(FieldType::String.surreal_type_name(true), "option<string>");
        assert_eq!(FieldType::Int.base_surreal_type_name(), "int");
        assert_eq!(FieldType::Float.base_surreal_type_name(), "float");
        assert_eq!(FieldType::Bool.base_surreal_type_name(), "bool");
        assert_eq!(FieldType::Datetime.base_surreal_type_name(), "datetime");
        assert_eq!(FieldType::Object.base_surreal_type_name(), "object");
        assert_eq!(
            FieldType::Array(Box::new(FieldType::String)).base_surreal_type_name(),
            "array<string>"
        );
        assert_eq!(FieldType::RecordId("User".into()).base_surreal_type_name(), "record<User>");
        assert_eq!(FieldType::Model("Post".into()).base_surreal_type_name(), "record<Post>");
        assert_eq!(FieldType::Custom("geometry".into()).base_surreal_type_name(), "geometry");
        assert_eq!(
            FieldType::String.surreal_type_name(true),
            "option<string>"
        );
        assert_eq!(FieldType::Int.surreal_type_name(false), "int");
    }

    #[test]
    fn on_delete_parses_and_formats() {
        assert_eq!(OnDeleteAction::parse("Cascade"), Some(OnDeleteAction::Cascade));
        assert_eq!(OnDeleteAction::parse("CASCADE"), Some(OnDeleteAction::Cascade));
        assert_eq!(OnDeleteAction::parse("cascade"), Some(OnDeleteAction::Cascade));
        assert_eq!(OnDeleteAction::Cascade.as_dsl(), "Cascade");
        assert_eq!(OnDeleteAction::Cascade.as_surreal(), "CASCADE");
        assert_eq!(OnDeleteAction::Unset.as_dsl(), "Unset");
        assert_eq!(OnDeleteAction::Unset.as_surreal(), "UNSET");
        assert_eq!(OnDeleteAction::parse("Unset"), Some(OnDeleteAction::Unset));
        assert_eq!(OnDeleteAction::Ignore.as_dsl(), "Ignore");
        assert_eq!(OnDeleteAction::Ignore.as_surreal(), "IGNORE");
        assert_eq!(OnDeleteAction::parse("Ignore"), Some(OnDeleteAction::Ignore));
        assert_eq!(OnDeleteAction::Reject.as_dsl(), "Reject");
        assert_eq!(OnDeleteAction::Reject.as_surreal(), "REJECT");
        assert_eq!(OnDeleteAction::parse("Reject"), Some(OnDeleteAction::Reject));
        assert_eq!(OnDeleteAction::parse("nope"), None);
    }

    #[test]
    fn link_model_name_defaults() {
        assert_eq!(FieldType::Model("User".into()).link_model_name(), Some("User"));
        assert_eq!(
            FieldType::Array(Box::new(FieldType::Model("Post".into()))).link_model_name(),
            Some("Post")
        );
        assert_eq!(FieldType::String.link_model_name(), None);
        assert_eq!(
            FieldType::Array(Box::new(FieldType::String)).link_model_name(),
            None
        );
        assert_eq!(FieldType::RecordId("User".into()).link_model_name(), None);
    }
}
