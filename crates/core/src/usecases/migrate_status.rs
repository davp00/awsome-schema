use std::collections::BTreeMap;
use std::sync::Arc;

use crate::domain::{DatabaseConfig, Datasource};
use crate::errors::DomainError;
use crate::ports::{MigrationLedger, MigrationStore};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationApplyState {
    Applied,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationStatusRow {
    pub name: String,
    pub state: MigrationApplyState,
    pub applied_at: Option<String>,
}

pub struct MigrateStatusInput {
    pub datasource: Datasource,
}

pub struct MigrateStatusOutput {
    pub migrations: Vec<MigrationStatusRow>,
    pub has_snapshot: bool,
    pub applied_count: usize,
    pub pending_count: usize,
}

pub struct MigrateStatusUseCase {
    migration_store: Arc<dyn MigrationStore>,
    ledger: Arc<dyn MigrationLedger>,
}

impl MigrateStatusUseCase {
    pub fn new(migration_store: Arc<dyn MigrationStore>, ledger: Arc<dyn MigrationLedger>) -> Self {
        Self { migration_store, ledger }
    }

    pub fn execute(&self, port: MigrateStatusInput) -> Result<MigrateStatusOutput, DomainError> {
        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        self.ledger.ensure_schema(&config)?;

        let applied_rows = self.ledger.list_applied(&config)?;
        let applied_by_name =
            applied_rows.into_iter().map(|row| (row.name.clone(), row)).collect::<BTreeMap<_, _>>();

        let migrations = self
            .migration_store
            .list_migrations()?
            .into_iter()
            .map(|name| {
                if let Some(row) = applied_by_name.get(&name) {
                    MigrationStatusRow {
                        name,
                        state: MigrationApplyState::Applied,
                        applied_at: row.applied_at.clone(),
                    }
                } else {
                    MigrationStatusRow {
                        name,
                        state: MigrationApplyState::Pending,
                        applied_at: None,
                    }
                }
            })
            .collect::<Vec<_>>();

        let applied_count =
            migrations.iter().filter(|row| row.state == MigrationApplyState::Applied).count();
        let pending_count = migrations.len() - applied_count;
        let has_snapshot = self.migration_store.load_last_snapshot()?.is_some();

        Ok(MigrateStatusOutput { migrations, has_snapshot, applied_count, pending_count })
    }
}
