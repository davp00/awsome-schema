use crate::domain::{DatabaseSchema, FieldType};
use crate::errors::DomainError;

pub fn validate_schema(schema: &DatabaseSchema) -> Result<(), DomainError> {
    if schema.datasource.provider.is_empty() {
        return Err(DomainError::ValidationError("datasource provider is required".to_owned()));
    }

    if schema.models.is_empty() && schema.edges.is_empty() {
        return Err(DomainError::ValidationError(
            "schema must define at least one model or edge".to_owned(),
        ));
    }

    for model in &schema.models {
        if model.fields.is_empty() {
            return Err(DomainError::ValidationError(format!(
                "model `{}` must define at least one field",
                model.name
            )));
        }

        let has_id = model.fields.iter().any(|field| field.is_id);
        if !has_id {
            return Err(DomainError::ValidationError(format!(
                "model `{}` must define an @id field",
                model.name
            )));
        }

        for field in &model.fields {
            if !field.is_id {
                continue;
            }
            if field.name.contains('.') {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` nested field `{}` cannot use @id",
                    model.name, field.name
                )));
            }
            match &field.field_type {
                FieldType::RecordId(target) if target == &model.name => {}
                _ => {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` field `{}` @id must reference the same model; use `id @id`",
                        model.name, field.name
                    )));
                }
            }
        }

        for field in &model.fields {
            if field.flexible && field.field_type != FieldType::Object {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` field `{}` @flexible requires type object",
                    model.name, field.name
                )));
            }

            if let Some(parent_path) = field.name.rsplit_once('.').map(|(parent, _)| parent) {
                let Some(parent) =
                    model.fields.iter().find(|candidate| candidate.name == parent_path)
                else {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` nested field `{}` requires parent field `{parent_path}`",
                        model.name, field.name
                    )));
                };
                if parent.field_type != FieldType::Object {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` parent field `{parent_path}` must be type object for nested field `{}`",
                        model.name, field.name
                    )));
                }
            }
        }
    }

    for edge in &schema.edges {
        if edge.in_model.is_empty() || edge.out_model.is_empty() {
            return Err(DomainError::ValidationError(format!(
                "edge `{}` must define both in and out models",
                edge.name
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::domain::{Datasource, Edge, Field, FieldType, Model, NamingConvention, TableMode};

    fn sample_schema() -> DatabaseSchema {
        DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: Some("env(\"SURREALDB_URL\")".to_owned()),
                namespace: Some("app".to_owned()),
                database: Some("main".to_owned()),
                extra: BTreeMap::new(),
            },
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: Vec::new(),
            models: vec![Model {
                name: "User".to_owned(),
                fields: vec![Field {
                    name: "id".to_owned(),
                    field_type: FieldType::RecordId("User".to_owned()),
                    optional: false,
                    unique: false,
                    is_id: true,
                    default_value: None,
                    default_always: false,
                    value_expression: None,
                    readonly: false,
                    flexible: false,
                    link_target: None,
                    relation_name: None,
                    attributes: BTreeMap::new(),
                }],
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            }],
            edges: Vec::new(),
        }
    }

    #[test]
    fn validates_minimal_schema() {
        validate_schema(&sample_schema()).expect("schema should be valid");
    }

    #[test]
    fn rejects_missing_provider() {
        let mut schema = sample_schema();
        schema.datasource.provider.clear();
        let error = validate_schema(&schema).expect_err("missing provider should fail");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_model_without_id_field() {
        let mut schema = sample_schema();
        schema.models[0].fields[0].is_id = false;
        let error = validate_schema(&schema).expect_err("missing id");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_edge_without_endpoints() {
        let mut schema = sample_schema();
        schema.edges.push(Edge {
            name: "Broken".to_owned(),
            in_model: String::new(),
            out_model: String::new(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("broken edge");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_empty_schema() {
        let schema = DatabaseSchema {
            datasource: sample_schema().datasource,
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: Vec::new(),
            models: Vec::new(),
            edges: Vec::new(),
        };
        let error = validate_schema(&schema).expect_err("empty");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_model_without_fields() {
        let mut schema = sample_schema();
        schema.models[0].fields.clear();
        let error = validate_schema(&schema).expect_err("no fields");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_wrong_id_field_type() {
        let mut schema = sample_schema();
        schema.models[0].fields[0].field_type = FieldType::RecordId("Post".to_owned());
        let error = validate_schema(&schema).expect_err("wrong id type");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn validates_schema_with_valid_edge() {
        let mut schema = sample_schema();
        schema.edges.push(Edge {
            name: "Follows".to_owned(),
            in_model: "User".to_owned(),
            out_model: "User".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        validate_schema(&schema).expect("valid edge");
    }

    #[test]
    fn validates_flexible_object_with_nested_fields() {
        let mut schema = sample_schema();
        schema.models[0].fields.extend([
            Field {
                name: "metadata".to_owned(),
                field_type: FieldType::Object,
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: true,
                link_target: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            },
            Field {
                name: "metadata.user_id".to_owned(),
                field_type: FieldType::Int,
                optional: true,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            },
        ]);
        validate_schema(&schema).expect("flexible nested fields");
    }

    #[test]
    fn rejects_nested_field_without_parent_object() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(Field {
            name: "metadata.user_id".to_owned(),
            field_type: FieldType::Int,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("missing parent");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_flexible_on_non_object_field() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(Field {
            name: "name".to_owned(),
            field_type: FieldType::String,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: true,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("flexible string");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }
}
