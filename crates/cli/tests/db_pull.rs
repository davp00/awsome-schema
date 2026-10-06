use std::sync::Arc;

use cli::commands::db;
use cli::output::Printer;
use schema_core::ports::SchemaIntrospector;
use schema_core::{DatabaseSchema, DomainError};

struct StubIntrospector;

impl SchemaIntrospector for StubIntrospector {
    fn introspect(
        &self,
        _config: &schema_core::DatabaseConfig,
        _preserve: &DatabaseSchema,
    ) -> Result<DatabaseSchema, DomainError> {
        let mut schema = DatabaseSchema::empty();
        schema.datasource.provider = "surrealdb".into();
        schema.models.push(schema_core::Model {
            name: "User".into(),
            fields: vec![schema_core::Field {
                name: "id".into(),
                field_type: schema_core::FieldType::RecordId("User".into()),
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
                attributes: Default::default(),
            }],
            table_mode: schema_core::TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: Default::default(),
        });
        Ok(schema)
    }
}

#[test]
fn run_pull_writes_schema_with_stub_introspector() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(
        &schema_path,
        r#"datasource db {
  provider = "surrealdb"
  url      = "127.0.0.1:8000"
}
model User { id @id }
"#,
    )
    .expect("write");

    let context = cli::test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
        Arc::new(StubIntrospector),
    )
    .expect("context");

    db::run_pull(&context, false, true, &Printer::new()).expect("pull");
    let written = std::fs::read_to_string(&schema_path).expect("read");
    assert!(written.contains("model User"));
}
