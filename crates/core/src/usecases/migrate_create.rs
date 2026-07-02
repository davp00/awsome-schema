use std::sync::Arc;

use chrono::Utc;

use crate::errors::DomainError;
use crate::ports::{FileSystemPort, MigrationStore, SchemaSource};

pub struct MigrateCreateInput {
    pub migrations_dir: String,
    pub name: String,
}

pub struct MigrateCreateOutput {
    pub migration_dir: String,
}

pub struct MigrateCreateUseCase {
    schema_source: Arc<dyn SchemaSource>,
    migration_store: Arc<dyn MigrationStore>,
    filesystem: Arc<dyn FileSystemPort>,
}

impl MigrateCreateUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        migration_store: Arc<dyn MigrationStore>,
        filesystem: Arc<dyn FileSystemPort>,
    ) -> Self {
        Self { schema_source, migration_store, filesystem }
    }

    pub fn execute(&self, port: MigrateCreateInput) -> Result<MigrateCreateOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;
        let migration_dir =
            format!("{}/{}_{}", port.migrations_dir, Utc::now().format("%Y%m%d%H%M%S"), port.name);

        self.filesystem.create_dir_all(&migration_dir)?;
        self.filesystem.write_string(
            &format!("{migration_dir}/migration.surql"),
            "-- Empty migration created manually\n",
        )?;
        self.migration_store.save_snapshot(&schema, &migration_dir)?;

        Ok(MigrateCreateOutput { migration_dir })
    }
}
