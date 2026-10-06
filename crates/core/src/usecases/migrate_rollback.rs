use std::collections::BTreeSet;
use std::sync::Arc;

use crate::domain::{DatabaseConfig, Datasource};
use crate::errors::DomainError;
use crate::ports::{DatabaseExecutor, FileSystemPort, MigrationLedger, MigrationStore};

pub struct MigrateRollbackInput {
    pub migrations_dir: String,
    pub datasource: Datasource,
    pub steps: usize,
}

#[derive(Debug)]
pub struct MigrateRollbackOutput {
    pub rolled_back: Vec<String>,
}

pub struct MigrateRollbackUseCase {
    migration_store: Arc<dyn MigrationStore>,
    filesystem: Arc<dyn FileSystemPort>,
    database: Arc<dyn DatabaseExecutor>,
    ledger: Arc<dyn MigrationLedger>,
}

impl MigrateRollbackUseCase {
    pub fn new(
        migration_store: Arc<dyn MigrationStore>,
        filesystem: Arc<dyn FileSystemPort>,
        database: Arc<dyn DatabaseExecutor>,
        ledger: Arc<dyn MigrationLedger>,
    ) -> Self {
        Self { migration_store, filesystem, database, ledger }
    }

    pub fn execute(&self, port: MigrateRollbackInput) -> Result<MigrateRollbackOutput, DomainError> {
        if port.steps == 0 {
            return Ok(MigrateRollbackOutput { rolled_back: Vec::new() });
        }

        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        self.ledger.ensure_schema(&config)?;

        let mut applied = self.ledger.list_applied(&config)?;
        applied.sort_by(|left, right| match (&right.applied_at, &left.applied_at) {
            (Some(right_at), Some(left_at)) if right_at != left_at => right_at.cmp(left_at),
            _ => right.name.cmp(&left.name),
        });

        let folder_names = self
            .migration_store
            .list_migrations()?
            .into_iter()
            .collect::<BTreeSet<_>>();

        let mut rolled_back = Vec::new();
        for record in applied.into_iter().take(port.steps) {
            if !folder_names.contains(&record.name) {
                return Err(DomainError::MigrationError(format!(
                    "applied migration `{}` has no local directory",
                    record.name
                )));
            }

            let down_path =
                format!("{}/{}/migration.down.surql", port.migrations_dir, record.name);
            if !self.filesystem.exists(&down_path) {
                return Err(DomainError::MigrationError(format!(
                    "migration `{}` has no migration.down.surql",
                    record.name
                )));
            }

            let script = self.filesystem.read_to_string(&down_path)?;
            if script.trim().is_empty() || script.trim().starts_with("-- No rollback") {
                return Err(DomainError::MigrationError(format!(
                    "migration `{}` has an empty down script",
                    record.name
                )));
            }

            self.database.execute_script(&config, &script)?;
            self.ledger.remove_applied(&config, &record.name)?;
            rolled_back.push(record.name);
        }

        Ok(MigrateRollbackOutput { rolled_back })
    }
}
