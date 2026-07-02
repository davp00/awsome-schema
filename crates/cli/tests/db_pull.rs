use std::sync::Arc;

use cli::commands::db;
use cli::output::Printer;
use schema_core::ports::SchemaIntrospector;
use schema_core::{DatabaseSchema, DomainError};

struct StubIntrospector;

impl SchemaIntrospector for StubIntrospector {
    fn introspect(&self) -> Result<DatabaseSchema, DomainError> {
        Ok(DatabaseSchema::empty())
    }
}

#[test]
fn run_pull_prints_success_with_stub_introspector() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(
        &schema_path,
        r#"datasource db { provider = "surrealdb" }
model User { id @id }"#,
    )
    .expect("write");

    let context = cli::test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
        Arc::new(StubIntrospector),
    )
    .expect("context");

    db::run_pull(&context, &Printer::new()).expect("pull");
}
