use std::collections::BTreeMap;

use crate::domain::{
    DatabaseSchema, Field, FieldType, LinkStorage, Model, ObjectTypeDefinition, ObjectTypeField,
    OnDeleteAction,
};
use crate::errors::DomainError;

pub fn normalize_schema(schema: &mut DatabaseSchema) -> Result<(), DomainError> {
    validate_object_type_names(schema)?;

    let object_types = schema.object_types.clone();
    let model_names: Vec<String> = schema.models.iter().map(|model| model.name.clone()).collect();

    for model in &mut schema.models {
        normalize_model_object_types(model, &object_types, &model_names)?;
    }

    resolve_link_pairs(schema)?;

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

#[derive(Clone)]
struct LinkRef {
    model_index: usize,
    field_index: usize,
    model_name: String,
    target: String,
    link_name: Option<String>,
    is_list: bool,
    optional: bool,
    has_on_delete: bool,
}

fn resolve_link_pairs(schema: &mut DatabaseSchema) -> Result<(), DomainError> {
    let mut links = Vec::new();
    for (model_index, model) in schema.models.iter().enumerate() {
        for (field_index, field) in model.fields.iter().enumerate() {
            let Some(target) = field.link_target.clone() else {
                continue;
            };
            links.push(LinkRef {
                model_index,
                field_index,
                model_name: model.name.clone(),
                target,
                link_name: field.link_name.clone(),
                is_list: field.is_list_link(),
                optional: field.optional,
                has_on_delete: field.on_delete.is_some(),
            });
        }
    }

    if links.is_empty() {
        return Ok(());
    }

    // Infer pair names when exactly one unpaired link exists between two models (both directions).
    infer_anonymous_pair_names(schema, &mut links)?;

    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, link) in links.iter().enumerate() {
        let Some(name) = &link.link_name else {
            // Solo stored link without pair / name
            continue;
        };
        by_name.entry(name.clone()).or_default().push(index);
    }

    for (name, indexes) in &by_name {
        if indexes.len() > 2 {
            return Err(DomainError::ValidationError(format!(
                "@link(\"{name}\") is used on more than two fields"
            )));
        }
        if indexes.len() == 1 {
            let link = &links[indexes[0]];
            set_link_storage(schema, link.model_index, link.field_index, LinkStorage::Stored);
            ensure_on_delete_default(schema, link.model_index, link.field_index);
            continue;
        }

        let left = &links[indexes[0]];
        let right = &links[indexes[1]];

        if left.is_list && right.is_list {
            return Err(DomainError::ValidationError(format!(
                "@link(\"{name}\") cannot be many-to-many on both sides; use an edge instead"
            )));
        }

        let (stored, computed) = resolve_stored_computed(left, right, name)?;
        set_link_storage(schema, stored.model_index, stored.field_index, LinkStorage::Stored);
        ensure_on_delete_default(schema, stored.model_index, stored.field_index);
        set_link_storage(schema, computed.model_index, computed.field_index, LinkStorage::Computed);
        let stored_field_name =
            schema.models[stored.model_index].fields[stored.field_index].name.clone();
        schema.models[computed.model_index].fields[computed.field_index].link_opposite_field =
            Some(stored_field_name);
        // Computed sides must not keep on_delete
        schema.models[computed.model_index].fields[computed.field_index].on_delete = None;
    }

    // Unnamed solo links: treat as stored
    for link in &links {
        if link.link_name.is_some() {
            continue;
        }
        let field = &schema.models[link.model_index].fields[link.field_index];
        if field.link_storage.is_none() {
            set_link_storage(schema, link.model_index, link.field_index, LinkStorage::Stored);
            ensure_on_delete_default(schema, link.model_index, link.field_index);
        }
    }

    Ok(())
}

fn infer_anonymous_pair_names(
    schema: &mut DatabaseSchema,
    links: &mut [LinkRef],
) -> Result<(), DomainError> {
    // Group unnamed links by unordered model pair
    let mut buckets: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (index, link) in links.iter().enumerate() {
        if link.link_name.is_some() {
            continue;
        }
        let a = link.model_name.clone();
        let b = link.target.clone();
        let key = if a <= b { (a, b) } else { (b, a) };
        buckets.entry(key).or_default().push(index);
    }

    for ((left_model, right_model), indexes) in buckets {
        if indexes.len() != 2 {
            continue;
        }
        let a = &links[indexes[0]];
        let b = &links[indexes[1]];
        // Must point at each other
        let cross = (a.target == b.model_name && b.target == a.model_name)
            || (a.model_name == b.model_name && a.target == b.target);
        if !cross {
            continue;
        }
        let inferred = format!("{left_model}{right_model}");
        for index in indexes {
            links[index].link_name = Some(inferred.clone());
            schema.models[links[index].model_index].fields[links[index].field_index].link_name =
                Some(inferred.clone());
        }
    }

    Ok(())
}

fn resolve_stored_computed<'a>(
    left: &'a LinkRef,
    right: &'a LinkRef,
    name: &str,
) -> Result<(&'a LinkRef, &'a LinkRef), DomainError> {
    if left.is_list != right.is_list {
        // singular = stored, list = computed
        if left.is_list {
            return Ok((right, left));
        }
        return Ok((left, right));
    }

    // 1-1: both singular
    match (left.has_on_delete, right.has_on_delete) {
        (true, false) => Ok((left, right)),
        (false, true) => Ok((right, left)),
        (true, true) => Err(DomainError::ValidationError(format!(
            "@link(\"{name}\") 1-1 pair has @onDelete on both sides; keep it on the stored side only"
        ))),
        (false, false) => match (left.optional, right.optional) {
            (false, true) => Ok((left, right)),
            (true, false) => Ok((right, left)),
            _ => Err(DomainError::ValidationError(format!(
                "@link(\"{name}\") 1-1 pair needs @onDelete on exactly one side to mark the stored field"
            ))),
        },
    }
}

fn set_link_storage(
    schema: &mut DatabaseSchema,
    model_index: usize,
    field_index: usize,
    storage: LinkStorage,
) {
    schema.models[model_index].fields[field_index].link_storage = Some(storage);
}

fn ensure_on_delete_default(schema: &mut DatabaseSchema, model_index: usize, field_index: usize) {
    let field = &mut schema.models[model_index].fields[field_index];
    if field.on_delete.is_none() {
        field.on_delete = Some(OnDeleteAction::Ignore);
    }
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
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
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
                        link_name: None,
                        on_delete: None,
                        link_storage: None,
                        link_opposite_field: None,
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
                        link_name: None,
                        on_delete: None,
                        link_storage: None,
                        link_opposite_field: None,
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
