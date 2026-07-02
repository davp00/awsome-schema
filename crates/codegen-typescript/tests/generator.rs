use std::collections::BTreeMap;

use codegen_typescript::TypeScriptGenerator;
use core::usecases::CodeGeneratorPort;
use core::{DatabaseSchema, Datasource, Field, FieldType, Model, NamingConvention, TableMode};

fn schema_with_field(field_type: FieldType, optional: bool) -> DatabaseSchema {
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
fn maps_all_field_types_to_typescript() {
    let generator = TypeScriptGenerator::new();
    let cases = [
        (FieldType::String, false, "string"),
        (FieldType::Int, false, "number"),
        (FieldType::Float, false, "number"),
        (FieldType::Bool, false, "boolean"),
        (FieldType::Datetime, false, "Date"),
        (FieldType::Object, false, "Record<string, unknown>"),
        (FieldType::Array(Box::new(FieldType::String)), false, "string[]"),
        (FieldType::RecordId("User".into()), false, "User"),
        (FieldType::Model("Post".into()), false, "Post"),
        (FieldType::Custom("GeoPoint".into()), false, "GeoPoint"),
        (FieldType::String, true, "value?: string"),
    ];

    for (field_type, optional, expected) in cases {
        let output =
            generator.generate(&schema_with_field(field_type, optional)).expect("generate");
        assert!(output.contains(expected), "expected `{expected}` in `{output}`");
    }
}

#[test]
fn default_generator_is_constructible() {
    use codegen::CodeGenerator;

    let generator = TypeScriptGenerator;
    assert_eq!(CodeGenerator::language(&generator), "typescript");
    let output = CodeGenerator::generate(&generator, &schema_with_field(FieldType::String, false))
        .expect("generate");
    assert!(output.contains("export type Sample"));
}
