use std::collections::BTreeMap;

use crate::domain::{
    DatabaseSchema, Field, FieldType, Model, ObjectTypeDefinition, ObjectTypeField,
};
use crate::errors::DomainError;

pub fn normalize_schema(schema: &mut DatabaseSchema) -> Result<(), DomainError> {
    validate_object_type_names(schema)?;

    let object_types = schema.object_types.clone();
    let model_names: Vec<String> = schema.models.iter().map(|model| model.name.clone()).collect();

    for model in &mut schema.models {
        normalize_model_object_types(model, &object_types, &model_names)?;
    }

    Ok(())
}

fn validate_object_type_names(schema: &DatabaseSchema) -> Result<(), DomainError> {
    let mut seen = std::collections::BTreeSet::new();

    for object_type in &schema.object_types {
        if !seen.insert(&object_type.name) {
            return Err(DomainError::ValidationError(format!(
                "object type `{}` is defined more than once",
                object_type.name
            )));
        }

        if schema.models.iter().any(|model| model.name == object_type.name) {
            return Err(DomainError::ValidationError(format!(
                "object type `{}` conflicts with model of the same name",
                object_type.name
            )));
        }

        for field in &object_type.fields {
            if field.name.contains('.') {
                return Err(DomainError::ValidationError(format!(
                    "object type `{}` field `{}` must be a simple name, not a nested path",
                    object_type.name, field.name
                )));
            }
        }
    }

    Ok(())
}

fn normalize_model_object_types(
    model: &mut Model,
    object_types: &[ObjectTypeDefinition],
    model_names: &[String],
) -> Result<(), DomainError> {
    let mut index = 0;
    while index < model.fields.len() {
        let field = model.fields[index].clone();
        if !should_expand_object_type(&field) {
            index += 1;
            continue;
        }

        let FieldType::Model(type_name) = &field.field_type else {
            index += 1;
            continue;
        };

        if let Some(object_type) =
            object_types.iter().find(|candidate| candidate.name == *type_name)
        {
            let mut parent = field;
            parent.field_type = FieldType::Object;
            parent.flexible = object_type.flexible;
            let nested = nested_fields_from_object_type(&parent.name, &object_type.fields);
            model.fields[index] = parent;
            for (offset, nested_field) in nested.into_iter().enumerate() {
                model.fields.insert(index + 1 + offset, nested_field);
            }
            index += object_type.fields.len() + 1;
            continue;
        }

        if model_names.iter().any(|candidate| candidate == type_name) {
            index += 1;
            continue;
        }

        return Err(DomainError::ValidationError(format!(
            "model `{}` field `{}` references unknown type `{type_name}`",
            model.name, field.name
        )));
    }

    Ok(())
}

fn should_expand_object_type(field: &Field) -> bool {
    matches!(
        &field.field_type,
        FieldType::Model(_) if field.link_target.is_none() && field.relation_name.is_none()
    )
}

fn nested_fields_from_object_type(parent: &str, fields: &[ObjectTypeField]) -> Vec<Field> {
    fields
        .iter()
        .map(|field| Field {
            name: format!("{parent}.{}", field.name),
            field_type: field.field_type.clone(),
            optional: field.optional,
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
        })
        .collect()
}

#[must_use]
pub fn nested_fields_from_object_body(parent: &str, fields: &[ObjectTypeField]) -> Vec<Field> {
    nested_fields_from_object_type(parent, fields)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::domain::{Datasource, NamingConvention, TableMode};

    fn sample_schema() -> DatabaseSchema {
        DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: vec![ObjectTypeDefinition {
                name: "UserMetadata".to_owned(),
                flexible: true,
                fields: vec![
                    ObjectTypeField {
                        name: "user_id".to_owned(),
                        field_type: FieldType::Int,
                        optional: true,
                    },
                    ObjectTypeField {
                        name: "source".to_owned(),
                        field_type: FieldType::String,
                        optional: false,
                    },
                ],
            }],
            models: vec![Model {
                name: "User".to_owned(),
                fields: vec![
                    Field {
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
                    },
                    Field {
                        name: "metadata".to_owned(),
                        field_type: FieldType::Model("UserMetadata".to_owned()),
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
                    },
                ],
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            }],
            edges: Vec::new(),
        }
    }

    #[test]
    fn expands_object_type_reference_into_nested_fields() {
        let mut schema = sample_schema();
        normalize_schema(&mut schema).expect("normalize");

        let user = &schema.models[0];
        let metadata = user.fields.iter().find(|field| field.name == "metadata").unwrap();
        assert!(metadata.flexible);
        assert_eq!(metadata.field_type, FieldType::Object);
        assert!(user.fields.iter().any(|field| field.name == "metadata.user_id"));
        assert!(user.fields.iter().any(|field| field.name == "metadata.source"));
    }
}
