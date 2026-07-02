use std::fs;
use std::path::Path;
use std::sync::Arc;

use schema_core::{DatabaseSchema, DomainError, FileSystemPort, MigrationStore};

pub struct MigrationStoreAdapter {
    filesystem: Arc<dyn FileSystemPort>,
    migrations_dir: String,
}

impl MigrationStoreAdapter {
    pub fn new(filesystem: Arc<dyn FileSystemPort>, migrations_dir: String) -> Self {
        Self { filesystem, migrations_dir }
    }
}

impl MigrationStore for MigrationStoreAdapter {
    fn load_last_snapshot(&self) -> Result<Option<DatabaseSchema>, DomainError> {
        let migrations = self.list_migrations()?;
        let Some(latest) = migrations.last() else {
            return Ok(None);
        };

        let snapshot_path = format!("{}/{latest}/snapshot.json", self.migrations_dir);
        if !self.filesystem.exists(&snapshot_path) {
            return Ok(None);
        }

        let content = self.filesystem.read_to_string(&snapshot_path)?;
        let schema = serde_json::from_str(&content)
            .map_err(|error| DomainError::MigrationError(error.to_string()))?;
        Ok(Some(schema))
    }

    fn save_snapshot(
        &self,
        schema: &DatabaseSchema,
        migration_dir: &str,
    ) -> Result<(), DomainError> {
        let content = serde_json::to_string_pretty(schema)
            .map_err(|error| DomainError::MigrationError(error.to_string()))?;
        self.filesystem.write_string(&format!("{migration_dir}/snapshot.json"), &content)
    }

    fn list_migrations(&self) -> Result<Vec<String>, DomainError> {
        let path = Path::new(&self.migrations_dir);
        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut entries = fs::read_dir(path)
            .map_err(|error| DomainError::MigrationError(error.to_string()))?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        entries.sort();
        Ok(entries)
    }
}
