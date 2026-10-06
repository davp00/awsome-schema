use std::collections::BTreeSet;
use std::sync::Arc;

use crate::domain::{DatabaseConfig, Datasource};
use crate::errors::DomainError;
use crate::ports::{DatabaseExecutor, FileSystemPort, MigrationLedger, MigrationStore};

pub struct MigrateApplyInput {
    pub migrations_dir: String,
    pub datasource: Datasource,
}

pub struct MigrateApplyOutput {
    pub applied: usize,
    pub skipped: usize,
}

pub struct MigrateApplyUseCase {
    migration_store: Arc<dyn MigrationStore>,
    filesystem: Arc<dyn FileSystemPort>,
    database: Arc<dyn DatabaseExecutor>,
    ledger: Arc<dyn MigrationLedger>,
}

impl MigrateApplyUseCase {
    pub fn new(
        migration_store: Arc<dyn MigrationStore>,
        filesystem: Arc<dyn FileSystemPort>,
        database: Arc<dyn DatabaseExecutor>,
        ledger: Arc<dyn MigrationLedger>,
    ) -> Self {
        Self { migration_store, filesystem, database, ledger }
    }

    pub fn execute(&self, port: MigrateApplyInput) -> Result<MigrateApplyOutput, DomainError> {
        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        self.ledger.ensure_schema(&config)?;

        let applied_names = self
            .ledger
            .list_applied(&config)?
            .into_iter()
            .map(|row| row.name)
            .collect::<BTreeSet<_>>();

        let migrations = self.migration_store.list_migrations()?;
        let mut applied = 0usize;
        let mut skipped = 0usize;

        for migration in migrations {
            if applied_names.contains(&migration) {
                skipped += 1;
                continue;
            }

            let migration_path = format!("{}/{migration}/migration.surql", port.migrations_dir);
            if !self.filesystem.exists(&migration_path) {
                continue;
            }

            let script = self.filesystem.read_to_string(&migration_path)?;
            if script.trim().is_empty() {
                continue;
            }

            self.database.execute_script(&config, &script)?;
            let checksum = checksum_of(&script);
            self.ledger.record_applied(&config, &migration, &checksum)?;
            applied += 1;
        }

        Ok(MigrateApplyOutput { applied, skipped })
    }
}

pub fn checksum_of(script: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    script.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
