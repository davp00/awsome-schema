use core::{DatabaseSchema, FieldType};

use codegen_rust::RustGenerator;
use core::usecases::CodeGeneratorPort;

fn schema_with_field(field_type: FieldType, optional: bool) -> DatabaseSchema {
    use std::collections::BTreeMap;

    use core::{Datasource, Field, Model, NamingConvention, TableMode};

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
        object_types: Vec::new(),
        models: vec![Model {
            name: "Sample".to_owned(),
            fields: vec![Field {
                name: "value".to_owned(),
                field_type,
                optional,
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
                object_type_name: None,
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
#[allow(clippy::default_constructed_unit_structs)]
fn rust_generator_default_and_language() {
    use codegen::CodeGenerator;
    use codegen_rust::RustGenerator;

    let generator = RustGenerator::default();
    assert_eq!(generator.language(), "rust");
    assert_eq!(CodeGenerator::language(&generator), "rust");
}

#[test]
fn maps_all_field_types_to_rust() {
    let generator = RustGenerator::new();
    let cases = [
        (FieldType::Int, false, "i64"),
        (FieldType::Float, false, "f64"),
        (FieldType::Bool, false, "bool"),
        (FieldType::Datetime, false, "pub value: String,"),
        (FieldType::Object, false, "pub value: Value,"),
        (FieldType::Array(Box::new(FieldType::String)), false, "Vec<String>"),
        (FieldType::String, true, "Option<String>"),
        (FieldType::Custom("MyType".into()), false, "MyType"),
    ];

    for (field_type, optional, expected) in cases {
        let output =
            generator.generate(&schema_with_field(field_type, optional)).expect("generate");
        assert!(output.contains(expected), "expected `{expected}` in `{output}`");
    }

    let record_output = generator
        .generate(&schema_with_field(FieldType::RecordId("User".into()), false))
        .expect("record");
    assert!(record_output.contains("String"));

    let model_output = generator
        .generate(&schema_with_field(FieldType::Model("User".into()), false))
        .expect("model");
    assert!(model_output.contains("String"));
}

#[test]
fn emits_dual_shapes_edges_and_delegates() {
    use std::collections::BTreeMap;

    use core::{Datasource, Edge, Field, Model, NamingConvention, TableMode};

    let field = |name: &str, field_type: FieldType, optional: bool, unique: bool| Field {
        name: name.to_owned(),
        field_type,
        optional,
        unique,
        is_id: name == "id",
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
        object_type_name: None,
        relation_name: None,
        attributes: BTreeMap::new(),
    };
    let mut author = field("author", FieldType::Model("User".into()), false, false);
    author.link_target = Some("User".into());
    author.link_storage = Some(core::LinkStorage::Stored);
    let mut liked =
        field("liked", FieldType::Array(Box::new(FieldType::Model("Post".into()))), false, false);
    liked.relation_name = Some("Likes".into());
    let schema = DatabaseSchema {
        datasource: Datasource {
            provider: "surrealdb".to_owned(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        },
        naming: NamingConvention::default(),
        generators: Vec::new(),
        object_types: Vec::new(),
        models: vec![
            Model {
                name: "User".to_owned(),
                fields: vec![
                    field("id", FieldType::String, false, true),
                    field("email", FieldType::String, false, true),
                    liked,
                ],
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            },
            Model {
                name: "Post".to_owned(),
                fields: vec![
                    field("id", FieldType::String, false, true),
                    field("title", FieldType::String, false, false),
                    author,
                ],
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            },
        ],
        edges: vec![Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "Post".to_owned(),
            fields: vec![field("score", FieldType::Int, false, false)],
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: BTreeMap::new(),
        }],
    };
    let output = CodeGeneratorPort::generate(&RustGenerator::new(), &schema).expect("generate");
    for needle in [
        "pub struct User {",
        "pub struct UserSelected {",
        "pub struct Likes {",
        "pub struct LikesSelected {",
        "pub struct UserWhere {",
        "pub struct UserSelect {",
        "pub struct UserOrderBy {",
        "fn create_client",
        "fn find_unique",
        "fn find_many",
        "fn create_many",
        "fn group_by",
        "fn transaction",
        "fn live(",
        "fn query_raw",
        "fn execute_raw",
        "relate_edge",
        "r#in",
        "r#out",
    ] {
        assert!(output.contains(needle), "missing `{needle}`");
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/generated_client.rs");
    std::fs::write(path, output).expect("write generated client");
}

#[test]
fn rejects_unimplemented_rust_provider() {
    let mut schema = schema_with_field(FieldType::String, false);
    schema.datasource.provider = "postgres".to_owned();
    let error = CodeGeneratorPort::generate(&RustGenerator::new(), &schema)
        .expect_err("postgres is not implemented");
    let message = error.to_string();
    assert!(message.contains("postgres"), "{message}");
    assert!(message.contains("surrealdb"), "{message}");
    assert!(!message.contains("pub struct"), "{message}");
}
