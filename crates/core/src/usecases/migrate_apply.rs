use std::sync::Arc;

use crate::domain::{DatabaseConfig, Datasource};
use crate::errors::DomainError;
use crate::ports::{DatabaseExecutor, FileSystemPort, MigrationStore};

pub struct MigrateApplyInput {
    pub migrations_dir: String,
    pub datasource: Datasource,
}

pub struct MigrateApplyOutput {
    pub applied: usize,
}

pub struct MigrateApplyUseCase {
    migration_store: Arc<dyn MigrationStore>,
    filesystem: Arc<dyn FileSystemPort>,
    database: Arc<dyn DatabaseExecutor>,
}

impl MigrateApplyUseCase {
    pub fn new(
        migration_store: Arc<dyn MigrationStore>,
        filesystem: Arc<dyn FileSystemPort>,
        database: Arc<dyn DatabaseExecutor>,
    ) -> Self {
        Self { migration_store, filesystem, database }
    }

    pub fn execute(&self, port: MigrateApplyInput) -> Result<MigrateApplyOutput, DomainError> {
        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        let migrations = self.migration_store.list_migrations()?;
        let mut applied = 0usize;

        for migration in migrations {
            let migration_path = format!("{}/{migration}/migration.surql", port.migrations_dir);
            if !self.filesystem.exists(&migration_path) {
                continue;
            }

            let script = self.filesystem.read_to_string(&migration_path)?;
            if script.trim().is_empty() {
                continue;
            }

            self.database.execute_script(&config, &script)?;
            applied += 1;
        }

        Ok(MigrateApplyOutput { applied })
    }
}
