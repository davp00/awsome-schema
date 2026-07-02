use core::DomainError;

#[test]
fn domain_error_codes_cover_all_variants() {
    assert_eq!(DomainError::SchemaNotFound("x".into()).code(), "SCHEMA_NOT_FOUND");
    assert_eq!(DomainError::SchemaReadFailed("x".into()).code(), "SCHEMA_READ_FAILED");
    assert_eq!(DomainError::WriteFailed("x".into()).code(), "WRITE_FAILED");
    assert_eq!(DomainError::ParseError("x".into()).code(), "PARSE_ERROR");
    assert_eq!(DomainError::ValidationError("x".into()).code(), "VALIDATION_ERROR");
    assert_eq!(DomainError::MigrationError("x".into()).code(), "MIGRATION_ERROR");
    assert_eq!(DomainError::RenderError("x".into()).code(), "RENDER_ERROR");
    assert_eq!(DomainError::CodegenError("x".into()).code(), "CODEGEN_ERROR");
    assert_eq!(DomainError::DatabaseError("x".into()).code(), "DATABASE_ERROR");
    assert_eq!(DomainError::UnsupportedProvider("x".into()).code(), "UNSUPPORTED_PROVIDER");
    assert_eq!(DomainError::NotImplemented("x".into()).code(), "NOT_IMPLEMENTED");
}

#[test]
fn db_pull_use_case_delegates_to_introspector() {
    use core::ports::SchemaIntrospector;
    use core::usecases::{DbPullInput, DbPullUseCase};
    use std::sync::Arc;

    struct Stub;
    impl SchemaIntrospector for Stub {
        fn introspect(&self) -> Result<core::DatabaseSchema, DomainError> {
            Ok(core::DatabaseSchema::empty())
        }
    }

    let output = DbPullUseCase::new(Arc::new(Stub)).execute(DbPullInput).expect("pull");
    assert!(output.schema.models.is_empty());
}
