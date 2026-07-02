use crate::domain::DatabaseSchema;
use crate::errors::DomainError;

pub trait MigrationStore: Send + Sync {
    fn load_last_snapshot(&self) -> Result<Option<DatabaseSchema>, DomainError>;
    fn save_snapshot(
        &self,
        schema: &DatabaseSchema,
        migration_dir: &str,
    ) -> Result<(), DomainError>;
    fn list_migrations(&self) -> Result<Vec<String>, DomainError>;
}
