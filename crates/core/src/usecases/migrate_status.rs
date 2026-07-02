use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::MigrationStore;

pub struct MigrateStatusInput;

pub struct MigrateStatusOutput {
    pub migrations: Vec<String>,
    pub has_snapshot: bool,
}

pub struct MigrateStatusUseCase {
    migration_store: Arc<dyn MigrationStore>,
}

impl MigrateStatusUseCase {
    pub fn new(migration_store: Arc<dyn MigrationStore>) -> Self {
        Self { migration_store }
    }

    pub fn execute(&self, _port: MigrateStatusInput) -> Result<MigrateStatusOutput, DomainError> {
        let migrations = self.migration_store.list_migrations()?;
        let has_snapshot = self.migration_store.load_last_snapshot()?.is_some();

        Ok(MigrateStatusOutput { migrations, has_snapshot })
    }
}
