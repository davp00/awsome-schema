use crate::domain::DatabaseSchema;
use crate::errors::DomainError;

pub trait SchemaIntrospector: Send + Sync {
    fn introspect(&self) -> Result<DatabaseSchema, DomainError>;
}
