use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("schema file not found: {0}")]
    SchemaNotFound(String),

    #[error("failed to read schema: {0}")]
    SchemaReadFailed(String),

    #[error("failed to write file: {0}")]
    WriteFailed(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("migration error: {0}")]
    MigrationError(String),

    #[error("render error: {0}")]
    RenderError(String),

    #[error("codegen error: {0}")]
    CodegenError(String),

    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("unsupported provider: {0}")]
    UnsupportedProvider(String),

    #[error("not implemented: {0}")]
    NotImplemented(String),
}

impl DomainError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SchemaNotFound(_) => "SCHEMA_NOT_FOUND",
            Self::SchemaReadFailed(_) => "SCHEMA_READ_FAILED",
            Self::WriteFailed(_) => "WRITE_FAILED",
            Self::ParseError(_) => "PARSE_ERROR",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::MigrationError(_) => "MIGRATION_ERROR",
            Self::RenderError(_) => "RENDER_ERROR",
            Self::CodegenError(_) => "CODEGEN_ERROR",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::UnsupportedProvider(_) => "UNSUPPORTED_PROVIDER",
            Self::NotImplemented(_) => "NOT_IMPLEMENTED",
        }
    }
}
