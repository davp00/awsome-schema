use crate::domain::DatabaseSchema;
use crate::errors::DomainError;

pub trait SchemaSource: Send + Sync {
    fn load_schema(&self) -> Result<DatabaseSchema, DomainError>;
    fn load_raw(&self) -> Result<String, DomainError>;
}
