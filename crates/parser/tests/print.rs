use std::collections::BTreeMap;

use core::{
    Datasource, Edge, Field, FieldType, Generator, Index, Model, NamingCase, NamingConvention,
    ObjectTypeDefinition, ObjectTypeField, OnDeleteAction, TableMode, VectorDist,
};
use parser::{parse, print_config_blocks, print_edge_block, print_model_block, print_schema};

const EXAMPLE: &str = include_str!("../../../examples/awesome.schema");

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

#[test]
fn print_schema_round_trips_example() {
    let parsed = parse(EXAMPLE).expect("parse");
    let printed = print_schema(&parsed);
    let again = parse(&printed).expect("re-parse");
    assert_eq!(parsed.models.len(), again.models.len());
    assert_eq!(parsed.edges.len(), again.edges.len());
    assert_eq!(parsed.datasource.provider, again.datasource.provider);
}

#[test]
fn print_config_blocks_covers_naming_generators_object_types_and_extra() {
    let mut schema = parse(EXAMPLE).expect("parse");
    schema.naming = NamingConvention {
        tables: NamingCase::PascalCase,
        fields: Some(NamingCase::CamelCase),
    };
    schema.datasource.extra.insert("timeout".to_owned(), "30s".to_owned());
    schema.generators.push(Generator {
        provider: "rust".to_owned(),
        output: "./rust".to_owned(),
        extra: BTreeMap::from([("edition".to_owned(), "2021".to_owned())]),
    });
    schema.object_types.push(ObjectTypeDefinition {
        name: "ExtraMeta".to_owned(),
        flexible: false,
        fields: vec![ObjectTypeField {
            name: "flag".to_owned(),
            field_type: FieldType::Bool,
            optional: true,
        }],
    });

    let printed = print_config_blocks(&schema);
    assert!(printed.contains("tables = \"PascalCase\""));
    assert!(printed.contains("fields = \"camelCase\""));
    assert!(printed.contains("timeout = \"30s\""));
    assert!(printed.contains("provider = \"rust\""));
    assert!(printed.contains("edition = \"2021\""));
    assert!(printed.contains("type ExtraMeta"));
    assert!(printed.contains("flag bool?"));
}

#[test]
fn print_naming_case_variants() {
    for (case, expected) in [
        (NamingCase::SnakeCase, "snake_case"),
        (NamingCase::CamelCase, "camelCase"),
        (NamingCase::PascalCase, "PascalCase"),
        (NamingCase::KebabCase, "kebab-case"),
        (NamingCase::Lowercase, "lowercase"),
    ] {
        let schema = core::DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            naming: NamingConvention { tables: case, fields: Some(case) },
            generators: Vec::new(),
            object_types: Vec::new(),
            models: Vec::new(),
            edges: Vec::new(),
        };
        let printed = print_config_blocks(&schema);
        if case == NamingCase::SnakeCase {
            // SnakeCase tables alone is omitted unless fields is set — fields forces print.
            assert!(printed.contains(&format!("fields = \"{expected}\"")));
        } else {
            assert!(printed.contains(&format!("tables = \"{expected}\"")));
        }
    }
}

#[test]
fn print_model_block_covers_indexes_defaults_links_and_types() {
    let mut active = bare_field("active", FieldType::Bool);
    active.default_value = Some("true".to_owned());
    active.default_always = true;

    let custom = bare_field("geo", FieldType::Custom("geometry".to_owned()));
    let record = bare_field("other", FieldType::RecordId("User".to_owned()));

    let mut author = bare_field("author", FieldType::Model("User".to_owned()));
    author.link_target = Some("User".to_owned());
    author.link_name = Some("PostAuthor".to_owned());
    author.on_delete = Some(OnDeleteAction::Cascade);

    let mut solo = bare_field("owner", FieldType::Model("User".to_owned()));
    solo.link_target = Some("User".to_owned());

    let mut id = bare_field("id", FieldType::RecordId("Post".to_owned()));
    id.is_id = true;

    let model = Model {
        name: "Post".to_owned(),
        fields: vec![id, active, custom, record, author, solo],
        table_mode: TableMode::Schemaless,
        permissions: Some("FULL".to_owned()),
        indexes: vec![
            Index {
                name: None,
                fields: vec!["active".to_owned()],
                unique: false,
                fulltext: true,
                fulltext_analyzer: Some("english".to_owned()),
                vector: false,
                vector_dimension: None,
                vector_dist: None,
            },
            Index {
                name: None,
                fields: vec!["geo".to_owned()],
                unique: false,
                fulltext: true,
                fulltext_analyzer: None,
                vector: true,
                vector_dimension: Some(128),
                vector_dist: Some(VectorDist::Cosine),
            },
            Index {
                name: None,
                fields: vec!["other".to_owned()],
                unique: false,
                fulltext: false,
                fulltext_analyzer: None,
                vector: true,
                vector_dimension: None,
                vector_dist: None,
            },
        ],
        attributes: BTreeMap::new(),
    };

    let printed = print_model_block(&model);
    assert!(printed.contains("@defaultAlways(true)"));
    assert!(printed.contains("active bool"));
    assert!(printed.contains("geo geometry"));
    assert!(printed.contains("other record<User>"));
    assert!(printed.contains("@link(\"PostAuthor\")"));
    assert!(printed.contains("@onDelete(Cascade)"));
    assert!(printed.contains("owner User @link"));
    assert!(printed.contains("@@table(schemaless)"));
    assert!(printed.contains("@fulltext(\"english\")"));
    assert!(printed.contains("@fulltext"));
    assert!(printed.contains("@vector(128)"));
    assert!(printed.contains("@dist(Cosine)"));
    assert!(printed.contains("@vector\n") || printed.contains("@vector\r") || printed.contains("@@index([other]) @vector"));
}

#[test]
fn print_edge_block_covers_fields_and_mode() {
    let edge = Edge {
        name: "Likes".to_owned(),
        in_model: "User".to_owned(),
        out_model: "Post".to_owned(),
        fields: vec![bare_field("score", FieldType::Int)],
        table_mode: TableMode::Schemaless,
        permissions: Some("NONE".to_owned()),
        attributes: BTreeMap::new(),
    };
    let printed = print_edge_block(&edge);
    assert!(printed.contains("edge Likes"));
    assert!(printed.contains("in  User"));
    assert!(printed.contains("out Post"));
    assert!(printed.contains("score int"));
    assert!(printed.contains("@@table(schemaless)"));
    assert!(printed.contains("@@permissions(\"NONE\")"));
}

#[test]
fn print_schema_includes_models_and_edges() {
    let schema = parse(EXAMPLE).expect("parse");
    let printed = print_schema(&schema);
    assert!(printed.contains("model User"));
    assert!(printed.contains("edge Likes"));
    assert!(print_model_block(&schema.models[0]).contains("model User"));
    assert!(print_edge_block(&schema.edges[0]).contains("edge Likes"));
}

#[test]
fn print_schema_includes_naming_and_plain_default() {
    let mut schema = parse(EXAMPLE).expect("parse");
    schema.naming = NamingConvention {
        tables: NamingCase::PascalCase,
        fields: None,
    };
    let mut status = bare_field("status", FieldType::String);
    status.default_value = Some("\"draft\"".to_owned());
    status.default_always = false;
    schema.models[0].fields.push(status);

    // Non-id RecordId pointing at another table prints `record<...>`.
    let external = bare_field("external", FieldType::RecordId("Session".to_owned()));
    schema.models[0].fields.push(external);

    // Non-id RecordId matching the model name omits the type (None path).
    let self_ref = bare_field("selfRef", FieldType::RecordId("User".to_owned()));
    schema.models[0].fields.push(self_ref);

    // Array<record<…>> exercises print_field_type's RecordId arm via recursion.
    let refs = bare_field(
        "refs",
        FieldType::Array(Box::new(FieldType::RecordId("Session".to_owned()))),
    );
    schema.models[0].fields.push(refs);

    let printed = print_schema(&schema);
    assert!(printed.contains("naming {"));
    assert!(printed.contains("tables = \"PascalCase\""));
    assert!(printed.contains("@default(\"draft\")"));
    assert!(!printed.contains("@defaultAlways(\"draft\")"));
    assert!(printed.contains("external record<Session>"));
    assert!(printed.contains("refs record<Session>[]"));
    assert!(printed.contains("selfRef\n") || printed.contains("selfRef\r"));
}
