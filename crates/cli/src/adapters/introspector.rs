use schema_core::{DatabaseSchema, DomainError, SchemaIntrospector};

/// SurrealDB introspection adapter (stub).
///
/// Runtime connectivity will use the official `surrealdb` Rust crate in a future milestone.
pub struct SurrealDbIntrospector;

impl SchemaIntrospector for SurrealDbIntrospector {
    fn introspect(&self) -> Result<DatabaseSchema, DomainError> {
        Err(DomainError::NotImplemented(
            "db pull requires SurrealDB connectivity (planned for a future milestone)".to_owned(),
        ))
    }
}
