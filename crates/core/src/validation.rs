use std::collections::BTreeMap;

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

        for index in &model.indexes {
            if index.fields.is_empty() {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` @@index requires at least one field",
                    model.name
                )));
            }
            for field_name in &index.fields {
                if !model.fields.iter().any(|field| field.name == *field_name) {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` @@index references unknown field `{field_name}`",
                        model.name
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

        if !schema.models.iter().any(|model| model.name == edge.in_model) {
            return Err(DomainError::ValidationError(format!(
                "edge `{}` in model `{}` does not exist",
                edge.name, edge.in_model
            )));
        }
        if !schema.models.iter().any(|model| model.name == edge.out_model) {
            return Err(DomainError::ValidationError(format!(
                "edge `{}` out model `{}` does not exist",
                edge.name, edge.out_model
            )));
        }
    }

    for model in &schema.models {
        for field in &model.fields {
            let Some(relation_name) = &field.relation_name else {
                continue;
            };
            // `table_name` already honors @@map / attributes.map.
            let matches_edge = schema.edges.iter().any(|edge| {
                edge.name == *relation_name || edge.table_name(&schema.naming) == *relation_name
            });
            if !matches_edge {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` field `{}` @relation(\"{relation_name}\") must name an existing edge",
                    model.name, field.name
                )));
            }
        }
    }

    validate_links(schema)?;

    Ok(())
}

fn validate_links(schema: &DatabaseSchema) -> Result<(), DomainError> {
    use crate::domain::LinkStorage;

    for model in &schema.models {
        for field in &model.fields {
            if field.link_target.is_some() && field.relation_name.is_some() {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` field `{}` cannot use both @link and @relation",
                    model.name, field.name
                )));
            }

            let Some(target) = &field.link_target else {
                if field.on_delete.is_some() {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` field `{}` @onDelete requires @link",
                        model.name, field.name
                    )));
                }
                continue;
            };

            if !schema.models.iter().any(|candidate| candidate.name == *target) {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` field `{}` @link targets unknown model `{target}`",
                    model.name, field.name
                )));
            }

            if field.field_type.link_model_name().is_none() {
                return Err(DomainError::ValidationError(format!(
                    "model `{}` field `{}` @link requires a model type (or model[])",
                    model.name, field.name
                )));
            }

            match field.link_storage {
                Some(LinkStorage::Computed) if field.on_delete.is_some() => {
                    return Err(DomainError::ValidationError(format!(
                        "model `{}` field `{}` is a computed @link and cannot use @onDelete",
                        model.name, field.name
                    )));
                }
                Some(LinkStorage::Stored) | None => {}
                Some(LinkStorage::Computed) => {}
            }
        }
    }

    // Named pairs: both sides exist when name is used twice; unpaired names with only computed are invalid
    let mut named: BTreeMap<&str, Vec<(&str, &str, Option<LinkStorage>, bool)>> = BTreeMap::new();
    for model in &schema.models {
        for field in &model.fields {
            let Some(name) = field.link_name.as_deref() else {
                continue;
            };
            named.entry(name).or_default().push((
                model.name.as_str(),
                field.name.as_str(),
                field.link_storage,
                field.is_list_link(),
            ));
        }
    }

    for (name, sides) in named {
        if sides.len() == 2 {
            let stored_count =
                sides.iter().filter(|(_, _, storage, _)| *storage == Some(LinkStorage::Stored)).count();
            if stored_count != 1 {
                return Err(DomainError::ValidationError(format!(
                    "@link(\"{name}\") must resolve to exactly one stored side"
                )));
            }
            sides
                .iter()
                .all(|(_, _, _, is_list)| *is_list)
                .then(|| {
                    DomainError::ValidationError(format!(
                        "@link(\"{name}\") cannot be many-to-many on both sides; use an edge instead"
                    ))
                })
                .map_or(Ok(()), Err)?;
        } else if sides.len() > 2 {
            return Err(DomainError::ValidationError(format!(
                "@link(\"{name}\") is used on more than two fields"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::domain::{
        Datasource, Edge, Field, FieldType, Index, LinkStorage, Model, NamingConvention,
        OnDeleteAction, TableMode,
    };

    fn bare_field(name: &str, field_type: FieldType) -> Field {
        Field {
            name: name.to_owned(),
            field_type,
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
        }
    }

    fn id_field(model: &str) -> Field {
        let mut field = bare_field("id", FieldType::RecordId(model.to_owned()));
        field.is_id = true;
        field
    }

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
    fn rejects_edge_with_unknown_endpoint_model() {
        let mut schema = sample_schema();
        schema.edges.push(Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Missing".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("unknown out");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_relation_field_without_matching_edge() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(Field {
            name: "posts".to_owned(),
            field_type: FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
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
            relation_name: Some("Missing".to_owned()),
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("missing edge");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn accepts_relation_field_naming_existing_edge() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![Field {
                name: "id".to_owned(),
                field_type: FieldType::RecordId("Post".to_owned()),
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
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        schema.edges.push(Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Post".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        schema.models[0].fields.push(Field {
            name: "posts".to_owned(),
            field_type: FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
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
            relation_name: Some("Likes".to_owned()),
            attributes: BTreeMap::new(),
        });
        validate_schema(&schema).expect("relation names edge");
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
                link_name: None,
                on_delete: None,
                link_storage: None,
                link_opposite_field: None,
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
                link_name: None,
                on_delete: None,
                link_storage: None,
                link_opposite_field: None,
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
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
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
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("flexible string");
        assert!(matches!(error, DomainError::ValidationError(_)));
    }

    #[test]
    fn rejects_nested_id_field() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(bare_field("metadata", FieldType::Object));
        let mut nested_id = bare_field("metadata.key", FieldType::RecordId("User".to_owned()));
        nested_id.is_id = true;
        schema.models[0].fields.push(nested_id);
        let error = validate_schema(&schema).expect_err("nested id");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("cannot use @id")));
    }

    #[test]
    fn rejects_nested_field_when_parent_is_not_object() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(bare_field("metadata", FieldType::String));
        schema.models[0].fields.push(bare_field("metadata.user_id", FieldType::Int));
        let error = validate_schema(&schema).expect_err("parent not object");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("must be type object")));
    }

    #[test]
    fn rejects_empty_index() {
        let mut schema = sample_schema();
        schema.models[0].indexes.push(Index {
            name: None,
            fields: Vec::new(),
            unique: false,
            fulltext: false,
            fulltext_analyzer: None,
            vector: false,
            vector_dimension: None,
            vector_dist: None,
        });
        let error = validate_schema(&schema).expect_err("empty index");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("at least one field")));
    }

    #[test]
    fn accepts_index_on_known_field() {
        let mut schema = sample_schema();
        schema.models[0].fields.push(bare_field("email", FieldType::String));
        schema.models[0].indexes.push(Index {
            name: None,
            fields: vec!["email".to_owned()],
            unique: true,
            fulltext: false,
            fulltext_analyzer: None,
            vector: false,
            vector_dimension: None,
            vector_dist: None,
        });
        validate_schema(&schema).expect("valid index");
    }

    #[test]
    fn accepts_single_sided_named_link() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        let mut author = bare_field("author", FieldType::Model("User".to_owned()));
        author.link_target = Some("User".to_owned());
        author.link_name = Some("Solo".to_owned());
        author.link_storage = Some(LinkStorage::Stored);
        author.on_delete = Some(OnDeleteAction::Cascade);
        schema.models[1].fields.push(author);
        validate_schema(&schema).expect("single-sided named link");
    }

    #[test]
    fn rejects_index_referencing_unknown_field() {
        let mut schema = sample_schema();
        schema.models[0].indexes.push(Index {
            name: None,
            fields: vec!["missing".to_owned()],
            unique: false,
            fulltext: false,
            fulltext_analyzer: None,
            vector: false,
            vector_dimension: None,
            vector_dist: None,
        });
        let error = validate_schema(&schema).expect_err("unknown index field");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("unknown field")));
    }

    #[test]
    fn rejects_edge_with_unknown_in_model() {
        let mut schema = sample_schema();
        schema.edges.push(Edge {
            name: "Owns".to_owned(),
            in_model: "Missing".to_owned(),
            out_model: "User".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        let error = validate_schema(&schema).expect_err("unknown in");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("in model")));
    }

    #[test]
    fn accepts_relation_field_matching_edge_table_name() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        schema.edges.push(Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Post".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        let mut posts = bare_field(
            "posts",
            FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
        );
        posts.relation_name = Some("likes".to_owned()); // snake_case table name
        schema.models[0].fields.push(posts);
        validate_schema(&schema).expect("relation by table_name");
    }

    #[test]
    fn accepts_relation_field_matching_edge_map_attribute() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        let mut edge = Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Post".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        };
        edge.attributes.insert("map".to_owned(), "user_likes_post".to_owned());
        schema.edges.push(edge);
        let mut posts = bare_field(
            "posts",
            FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
        );
        posts.relation_name = Some("user_likes_post".to_owned());
        schema.models[0].fields.push(posts);
        validate_schema(&schema).expect("relation by map");
    }

    #[test]
    fn rejects_field_with_both_link_and_relation() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        schema.edges.push(Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Post".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        });
        let mut field = bare_field("post", FieldType::Model("Post".to_owned()));
        field.link_target = Some("Post".to_owned());
        field.relation_name = Some("Likes".to_owned());
        schema.models[0].fields.push(field);
        let error = validate_schema(&schema).expect_err("both link and relation");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("both @link and @relation")));
    }

    #[test]
    fn rejects_on_delete_without_link() {
        let mut schema = sample_schema();
        let mut field = bare_field("name", FieldType::String);
        field.on_delete = Some(OnDeleteAction::Cascade);
        schema.models[0].fields.push(field);
        let error = validate_schema(&schema).expect_err("onDelete without link");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("@onDelete requires @link")));
    }

    #[test]
    fn rejects_link_targeting_unknown_model() {
        let mut schema = sample_schema();
        let mut field = bare_field("post", FieldType::Model("Post".to_owned()));
        field.link_target = Some("Post".to_owned());
        schema.models[0].fields.push(field);
        let error = validate_schema(&schema).expect_err("unknown link target");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("unknown model")));
    }

    #[test]
    fn rejects_link_on_non_model_field_type() {
        let mut schema = sample_schema();
        let mut field = bare_field("post", FieldType::String);
        field.link_target = Some("User".to_owned());
        schema.models[0].fields.push(field);
        let error = validate_schema(&schema).expect_err("non-model link");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("requires a model type")));
    }

    #[test]
    fn rejects_computed_link_with_on_delete() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        let mut field = bare_field("posts", FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))));
        field.link_target = Some("Post".to_owned());
        field.link_name = Some("Author".to_owned());
        field.link_storage = Some(LinkStorage::Computed);
        field.on_delete = Some(OnDeleteAction::Cascade);
        schema.models[0].fields.push(field);
        let error = validate_schema(&schema).expect_err("computed onDelete");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("computed @link")));
    }

    #[test]
    fn rejects_named_link_without_exactly_one_stored_side() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        let mut author = bare_field("author", FieldType::Model("User".to_owned()));
        author.link_target = Some("User".to_owned());
        author.link_name = Some("Author".to_owned());
        author.link_storage = Some(LinkStorage::Computed);
        let mut posts = bare_field(
            "posts",
            FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
        );
        posts.link_target = Some("Post".to_owned());
        posts.link_name = Some("Author".to_owned());
        posts.link_storage = Some(LinkStorage::Computed);
        schema.models[0].fields.push(posts);
        schema.models[1].fields.push(author);
        let error = validate_schema(&schema).expect_err("no stored side");
        assert!(matches!(
            error,
            DomainError::ValidationError(msg) if msg.contains("exactly one stored side")
        ));
    }

    #[test]
    fn rejects_named_link_on_more_than_two_fields() {
        let mut schema = sample_schema();
        let mut a = bare_field("a", FieldType::Model("User".to_owned()));
        a.link_target = Some("User".to_owned());
        a.link_name = Some("Self".to_owned());
        a.link_storage = Some(LinkStorage::Stored);
        let mut b = bare_field("b", FieldType::Model("User".to_owned()));
        b.link_target = Some("User".to_owned());
        b.link_name = Some("Self".to_owned());
        b.link_storage = Some(LinkStorage::Computed);
        let mut c = bare_field("c", FieldType::Model("User".to_owned()));
        c.link_target = Some("User".to_owned());
        c.link_name = Some("Self".to_owned());
        c.link_storage = Some(LinkStorage::Computed);
        schema.models[0].fields.extend([a, b, c]);
        let error = validate_schema(&schema).expect_err("more than two");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("more than two")));
    }

    #[test]
    fn rejects_named_many_to_many_link_pair() {
        let mut schema = sample_schema();
        schema.models.push(Model {
            name: "Post".to_owned(),
            fields: vec![id_field("Post")],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        let mut posts = bare_field(
            "posts",
            FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
        );
        posts.link_target = Some("Post".to_owned());
        posts.link_name = Some("Tagged".to_owned());
        posts.link_storage = Some(LinkStorage::Stored);
        let mut users = bare_field(
            "users",
            FieldType::Array(Box::new(FieldType::Model("User".to_owned()))),
        );
        users.link_target = Some("User".to_owned());
        users.link_name = Some("Tagged".to_owned());
        users.link_storage = Some(LinkStorage::Computed);
        schema.models[0].fields.push(posts);
        schema.models[1].fields.push(users);
        let error = validate_schema(&schema).expect_err("many-to-many");
        assert!(matches!(error, DomainError::ValidationError(msg) if msg.contains("many-to-many")));
    }
}
