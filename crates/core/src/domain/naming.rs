use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Field, FieldType, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NamingCase {
    #[default]
    SnakeCase,
    CamelCase,
    PascalCase,
    KebabCase,
    Lowercase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingConvention {
    pub tables: NamingCase,
    /// When `None`, field names are preserved as declared in the schema.
    pub fields: Option<NamingCase>,
}

impl Default for NamingConvention {
    fn default() -> Self {
        Self { tables: NamingCase::SnakeCase, fields: None }
    }
}

impl NamingCase {
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "snake_case" => Some(Self::SnakeCase),
            "camelCase" => Some(Self::CamelCase),
            "PascalCase" => Some(Self::PascalCase),
            "kebab-case" => Some(Self::KebabCase),
            "lowercase" => Some(Self::Lowercase),
            _ => None,
        }
    }

    #[must_use]
    pub fn apply(&self, name: &str) -> String {
        match self {
            Self::SnakeCase => to_snake_case(name),
            Self::CamelCase => to_camel_case(name),
            Self::PascalCase => to_pascal_case(name),
            Self::KebabCase => to_kebab_case(name),
            Self::Lowercase => name.to_ascii_lowercase(),
        }
    }
}

#[must_use]
pub fn mapped_name(
    name: &str,
    attributes: &BTreeMap<String, String>,
    case: Option<NamingCase>,
) -> String {
    if let Some(mapped) = attributes.get("map") {
        return mapped.clone();
    }
    match case {
        Some(convention) => convention.apply(name),
        None => name.to_owned(),
    }
}

pub struct NamingContext<'a> {
    naming: &'a NamingConvention,
    models: &'a [Model],
}

impl<'a> NamingContext<'a> {
    #[must_use]
    pub const fn new(naming: &'a NamingConvention, models: &'a [Model]) -> Self {
        Self { naming, models }
    }

    #[must_use]
    pub const fn convention(&self) -> &NamingConvention {
        self.naming
    }

    #[must_use]
    pub fn map_table_name(&self, name: &str, attributes: &BTreeMap<String, String>) -> String {
        mapped_name(name, attributes, Some(self.naming.tables))
    }

    #[must_use]
    pub fn table_name_for_model(&self, model: &Model) -> String {
        mapped_name(&model.name, &model.attributes, Some(self.naming.tables))
    }

    #[must_use]
    pub fn table_name_for_type(&self, type_name: &str) -> String {
        self.models.iter().find(|model| model.name == type_name).map_or_else(
            || self.naming.tables.apply(type_name),
            |model| self.table_name_for_model(model),
        )
    }

    #[must_use]
    pub fn field_name(&self, field: &Field) -> String {
        map_field_path(&field.name, &field.attributes, self.naming.fields)
    }

    #[must_use]
    pub fn field_name_str(&self, name: &str, attributes: &BTreeMap<String, String>) -> String {
        map_field_path(name, attributes, self.naming.fields)
    }

    #[must_use]
    pub fn surreal_type_name(&self, field_type: &FieldType, optional: bool) -> String {
        let inner = self.base_surreal_type_name(field_type);
        if optional { format!("option<{inner}>") } else { inner }
    }

    fn base_surreal_type_name(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::String => "string".to_owned(),
            FieldType::Int => "int".to_owned(),
            FieldType::Float => "float".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::Datetime => "datetime".to_owned(),
            FieldType::Object => "object".to_owned(),
            FieldType::Array(inner) => {
                format!("array<{}>", self.base_surreal_type_name(inner))
            }
            FieldType::RecordId(target) => {
                format!("record<{}>", self.table_name_for_type(target))
            }
            FieldType::Model(name) => format!("record<{}>", self.table_name_for_type(name)),
            FieldType::Custom(value) => value.clone(),
        }
    }
}

fn map_field_path(
    name: &str,
    attributes: &BTreeMap<String, String>,
    field_naming: Option<NamingCase>,
) -> String {
    let segments: Vec<&str> = name.split('.').collect();
    if segments.len() == 1 {
        return mapped_name(name, attributes, field_naming);
    }

    let last = segments.len() - 1;
    segments
        .into_iter()
        .enumerate()
        .map(|(index, segment)| {
            if index == last {
                mapped_name(segment, attributes, field_naming)
            } else {
                mapped_name(segment, &BTreeMap::new(), field_naming)
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn to_snake_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 4);

    for (i, ch) in input.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                let prev = input.chars().nth(i - 1);
                if prev.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
                    result.push('_');
                } else if prev.is_some_and(|c| c.is_ascii_uppercase()) {
                    let next = input.chars().nth(i + 1);
                    if next.is_some_and(|c| c.is_ascii_lowercase()) {
                        result.push('_');
                    }
                }
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch == '-' || ch == ' ' {
            result.push('_');
        } else {
            result.push(ch);
        }
    }

    result
}

fn to_camel_case(input: &str) -> String {
    let parts = split_identifier_parts(input);
    let mut result = String::new();

    for (index, part) in parts.iter().enumerate() {
        let lower = part.to_ascii_lowercase();
        if index == 0 {
            result.push_str(&lower);
        } else {
            let mut chars = lower.chars();
            // `split_identifier_parts` never yields empty parts.
            let first = chars.next().expect("non-empty identifier part");
            result.push(first.to_ascii_uppercase());
            result.extend(chars);
        }
    }

    result
}

fn to_pascal_case(input: &str) -> String {
    split_identifier_parts(input)
        .into_iter()
        .filter_map(|part| {
            let lower = part.to_ascii_lowercase();
            let mut chars = lower.chars();
            chars.next().map(|first| {
                let mut value = String::new();
                value.push(first.to_ascii_uppercase());
                value.extend(chars);
                value
            })
        })
        .collect()
}

fn to_kebab_case(input: &str) -> String {
    to_snake_case(input).replace('_', "-")
}

fn split_identifier_parts(input: &str) -> Vec<String> {
    to_snake_case(input).split('_').filter(|part| !part.is_empty()).map(ToOwned::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::TableMode;

    #[test]
    fn snake_case_converts_camel_case() {
        assert_eq!(NamingCase::SnakeCase.apply("createdAt"), "created_at");
        assert_eq!(NamingCase::SnakeCase.apply("User"), "user");
        assert_eq!(NamingCase::SnakeCase.apply("UserPost"), "user_post");
    }

    #[test]
    fn camel_case_converts_snake_case() {
        assert_eq!(NamingCase::CamelCase.apply("created_at"), "createdAt");
    }

    #[test]
    fn map_attribute_overrides_convention() {
        let mut attributes = BTreeMap::new();
        attributes.insert("map".to_owned(), "custom_table".to_owned());

        assert_eq!(mapped_name("User", &attributes, Some(NamingCase::SnakeCase)), "custom_table");
    }

    #[test]
    fn preserves_field_name_when_naming_not_set() {
        assert_eq!(mapped_name("createdAt", &BTreeMap::new(), None), "createdAt");
    }

    #[test]
    fn parse_all_naming_cases() {
        assert_eq!(NamingCase::parse("snake_case"), Some(NamingCase::SnakeCase));
        assert_eq!(NamingCase::parse("camelCase"), Some(NamingCase::CamelCase));
        assert_eq!(NamingCase::parse("PascalCase"), Some(NamingCase::PascalCase));
        assert_eq!(NamingCase::parse("kebab-case"), Some(NamingCase::KebabCase));
        assert_eq!(NamingCase::parse("lowercase"), Some(NamingCase::Lowercase));
        assert_eq!(NamingCase::parse("unknown"), None);
    }

    #[test]
    fn applies_all_naming_case_transforms() {
        assert_eq!(NamingCase::PascalCase.apply("user_post"), "UserPost");
        assert_eq!(NamingCase::KebabCase.apply("UserPost"), "user-post");
        assert_eq!(NamingCase::Lowercase.apply("UserPost"), "userpost");
        assert_eq!(NamingCase::SnakeCase.apply("HTTPRequest"), "http_request");
        assert_eq!(NamingCase::SnakeCase.apply("User-Post"), "user_post");
    }

    #[test]
    fn naming_context_resolves_table_and_field_names() {
        let naming =
            NamingConvention { tables: NamingCase::SnakeCase, fields: Some(NamingCase::CamelCase) };
        let models = vec![Model {
            name: "UserPost".to_owned(),
            fields: vec![Field {
                name: "created_at".to_owned(),
                field_type: FieldType::Datetime,
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: None,
                link_name: None,
                on_delete: None,
                link_storage: None,
                link_opposite_field: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }];
        let ctx = NamingContext::new(&naming, &models);

        assert_eq!(ctx.table_name_for_model(&models[0]), "user_post");
        assert_eq!(ctx.table_name_for_type("UserPost"), "user_post");
        assert_eq!(ctx.table_name_for_type("Unknown"), "unknown");
        assert_eq!(ctx.field_name(&models[0].fields[0]), "createdAt");
        assert_eq!(
            ctx.surreal_type_name(&FieldType::Array(Box::new(FieldType::String)), false),
            "array<string>"
        );
        assert_eq!(
            ctx.surreal_type_name(&FieldType::RecordId("UserPost".to_owned()), false),
            "record<user_post>"
        );
        assert_eq!(
            ctx.surreal_type_name(&FieldType::Model("UserPost".to_owned()), false),
            "record<user_post>"
        );
        assert_eq!(
            ctx.surreal_type_name(&FieldType::Custom("geo".to_owned()), true),
            "option<geo>"
        );
        assert_eq!(ctx.surreal_type_name(&FieldType::Int, false), "int");
        assert_eq!(ctx.surreal_type_name(&FieldType::Float, false), "float");
        assert_eq!(ctx.surreal_type_name(&FieldType::Bool, false), "bool");
        assert_eq!(ctx.surreal_type_name(&FieldType::Object, false), "object");
    }

    #[test]
    fn maps_dotted_field_paths_per_segment() {
        let naming =
            NamingConvention { tables: NamingCase::SnakeCase, fields: Some(NamingCase::SnakeCase) };
        let ctx = NamingContext::new(&naming, &[]);
        assert_eq!(ctx.field_name_str("metadata.userId", &BTreeMap::new()), "metadata.user_id");
    }
}
