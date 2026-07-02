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
fn rust_generator_default_and_language() {
    use codegen::CodeGenerator;
    use codegen_rust::RustGenerator;

    let generator = RustGenerator;
    assert_eq!(generator.language(), "rust");
}

#[test]
fn maps_all_field_types_to_rust() {
    let generator = RustGenerator::new();
    let cases = [
        (FieldType::Int, false, "i64"),
        (FieldType::Float, false, "f64"),
        (FieldType::Bool, false, "bool"),
        (FieldType::Datetime, false, "chrono::DateTime"),
        (FieldType::Object, false, "serde_json::Value"),
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
