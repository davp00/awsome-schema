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
