use std::collections::BTreeMap;

use core::domain::{Field, FieldType, Index, Model, NamingCase, NamingConvention, TableMode};

#[test]
fn resolved_name_uses_explicit_index_name() {
    let index = Index {
        name: Some("custom".to_owned()),
        fields: vec!["email".to_owned()],
        unique: false,
        fulltext: false,
        fulltext_analyzer: None,
        vector: false,
        vector_dimension: None,
        vector_dist: None,
    };
    let naming =
        NamingConvention { tables: NamingCase::SnakeCase, fields: Some(NamingCase::SnakeCase) };
    assert_eq!(index.resolved_name("user", &naming), "custom");
}

#[test]
fn resolved_name_builds_default_from_fields() {
    let index = Index {
        name: None,
        fields: vec!["createdAt".to_owned()],
        unique: false,
        fulltext: false,
        fulltext_analyzer: None,
        vector: false,
        vector_dimension: None,
        vector_dist: None,
    };
    let naming = NamingConvention::default();
    assert_eq!(index.resolved_name("user", &naming), "user_createdAt_idx");
}

#[test]
fn model_and_edge_table_names_use_naming_convention() {
    let naming = NamingConvention { tables: NamingCase::SnakeCase, fields: None };
    let model = Model {
        name: "UserPost".to_owned(),
        fields: Vec::new(),
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: Vec::new(),
        attributes: BTreeMap::new(),
    };
    assert_eq!(model.table_name(&naming), "user_post");

    let mut attributes = BTreeMap::new();
    attributes.insert("map".to_owned(), "custom_edge".to_owned());
    let edge = core::Edge {
        name: "UserLikes".to_owned(),
        in_model: "User".to_owned(),
        out_model: "Post".to_owned(),
        fields: vec![Field {
            name: "score".to_owned(),
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
        }],
        table_mode: TableMode::Schemafull,
        permissions: None,
        attributes,
    };
    assert_eq!(edge.table_name(&naming), "custom_edge");
}
