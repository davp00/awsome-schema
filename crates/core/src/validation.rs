use crate::domain::DatabaseSchema;
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
    use crate::domain::{Datasource, Field, FieldType, Model, TableMode};

    fn sample_schema() -> DatabaseSchema {
        DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: Some("env(\"SURREALDB_URL\")".to_owned()),
                namespace: Some("app".to_owned()),
                database: Some("main".to_owned()),
                extra: BTreeMap::new(),
            },
            generators: Vec::new(),
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
}
