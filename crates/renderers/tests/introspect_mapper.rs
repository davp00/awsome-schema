use std::collections::BTreeMap;

use core::{DatabaseSchema, Model, NamingCase, TableMode};
use renderers::{map_database_info, TableInfo};

#[test]
fn maps_minimal_user_fixture() {
    let mut preserve = DatabaseSchema::empty();
    preserve.datasource.provider = "surrealdb".into();
    preserve.naming.tables = NamingCase::SnakeCase;
    preserve.models.push(Model {
        name: "User".into(),
        fields: vec![],
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: vec![],
        attributes: BTreeMap::new(),
    });

    let mut tables = BTreeMap::new();
    tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
    let mut info = TableInfo::default();
    info.fields.insert("email".into(), "DEFINE FIELD email ON user TYPE string;".into());
    info.fields.insert("id".into(), "DEFINE FIELD id ON user TYPE record<user>;".into());
    info.indexes.insert(
        "user_email_unique".into(),
        "DEFINE INDEX user_email_unique ON user FIELDS email UNIQUE;".into(),
    );
    let mut table_infos = BTreeMap::new();
    table_infos.insert("user".into(), info);

    let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
    assert_eq!(pulled.models.len(), 1);
    assert_eq!(pulled.models[0].name, "User");
}

#[test]
fn maps_minimal_user_info_json_fixture() {
    let raw = include_str!("fixtures/minimal_user_info.json");
    let fixture: serde_json::Value = serde_json::from_str(raw).expect("json");
    let mut preserve = DatabaseSchema::empty();
    preserve.datasource.provider = "surrealdb".into();
    preserve.naming.tables = NamingCase::SnakeCase;
    preserve.models.push(Model {
        name: "User".into(),
        fields: vec![],
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: vec![],
        attributes: BTreeMap::new(),
    });

    let tables_obj = fixture["tables"].as_object().expect("tables");
    let mut tables = BTreeMap::new();
    for (name, value) in tables_obj {
        tables.insert(name.clone(), value.as_str().unwrap().to_owned());
    }
    let info_obj = fixture["table_info"]["user"].as_object().expect("table info");
    let mut info = TableInfo::default();
    for (name, value) in info_obj["fields"].as_object().expect("fields") {
        info.fields.insert(name.clone(), value.as_str().unwrap().to_owned());
    }
    for (name, value) in info_obj["indexes"].as_object().expect("indexes") {
        info.indexes.insert(name.clone(), value.as_str().unwrap().to_owned());
    }
    let mut table_infos = BTreeMap::new();
    table_infos.insert("user".into(), info);

    let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
    assert_eq!(pulled.models[0].name, "User");
}

#[test]
fn maps_relation_table_to_edge() {
    let mut preserve = DatabaseSchema::empty();
    preserve.datasource.provider = "surrealdb".into();
    preserve.naming.tables = NamingCase::SnakeCase;
    preserve.models.push(Model {
        name: "User".into(),
        fields: vec![],
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: vec![],
        attributes: BTreeMap::new(),
    });
    preserve.models.push(Model {
        name: "Post".into(),
        fields: vec![],
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: vec![],
        attributes: BTreeMap::new(),
    });

    let mut tables = BTreeMap::new();
    tables.insert(
        "likes".into(),
        "DEFINE TABLE likes TYPE RELATION IN user OUT post SCHEMAFULL;".into(),
    );
    let mut info = TableInfo::default();
    info.fields.insert(
        "in".into(),
        "DEFINE FIELD in ON likes TYPE record<user> PERMISSIONS FULL;".into(),
    );
    info.fields.insert(
        "out".into(),
        "DEFINE FIELD out ON likes TYPE record<post> PERMISSIONS FULL;".into(),
    );
    info.fields.insert("score".into(), "DEFINE FIELD score ON likes TYPE int;".into());
    let mut table_infos = BTreeMap::new();
    table_infos.insert("likes".into(), info);

    let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
    assert_eq!(pulled.edges.len(), 1);
    assert_eq!(pulled.edges[0].name, "Likes");
    assert_eq!(pulled.edges[0].in_model, "User");
    assert_eq!(pulled.edges[0].out_model, "Post");
    assert_eq!(pulled.edges[0].fields.len(), 1);
    assert_eq!(pulled.edges[0].fields[0].name, "score");
}
