use crate::domain::DatabaseSchema;
use crate::errors::DomainError;

pub trait SchemaRenderer: Send + Sync {
    fn render_schema(&self, schema: &DatabaseSchema) -> Result<String, DomainError>;
}
