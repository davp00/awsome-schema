use crate::domain::DatabaseSchema;
use crate::domain::database_config::DatabaseConfig;
use crate::errors::DomainError;

pub trait SchemaIntrospector: Send + Sync {
    fn introspect(
        &self,
        config: &DatabaseConfig,
        preserve: &DatabaseSchema,
    ) -> Result<DatabaseSchema, DomainError>;
}
