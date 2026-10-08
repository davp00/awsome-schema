use std::sync::Arc;

use chrono::Utc;

use crate::domain::{DatabaseSchema, require_surreal_provider};
use crate::errors::DomainError;
use crate::ports::{FileSystemPort, MigrationRenderer, MigrationStore, SchemaSource};

pub struct MigrateDevInput {
    pub migrations_dir: String,
    pub migration_name: Option<String>,
}

pub struct MigrateDevOutput {
    pub migration_dir: String,
    pub operation_count: usize,
    pub created: bool,
}

pub struct MigrateDevUseCase {
    schema_source: Arc<dyn SchemaSource>,
    migration_store: Arc<dyn MigrationStore>,
    migration_renderer: Arc<dyn MigrationRenderer>,
    filesystem: Arc<dyn FileSystemPort>,
    diff: Arc<dyn SchemaDiffPort>,
}

pub trait SchemaDiffPort: Send + Sync {
    fn diff(
        &self,
        from: Option<&crate::domain::DatabaseSchema>,
        to: &crate::domain::DatabaseSchema,
        name: &str,
    ) -> crate::domain::MigrationPlan;
}

impl MigrateDevUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        migration_store: Arc<dyn MigrationStore>,
        migration_renderer: Arc<dyn MigrationRenderer>,
        filesystem: Arc<dyn FileSystemPort>,
        diff: Arc<dyn SchemaDiffPort>,
    ) -> Self {
        Self { schema_source, migration_store, migration_renderer, filesystem, diff }
    }

    pub fn execute(&self, port: MigrateDevInput) -> Result<MigrateDevOutput, DomainError> {
        let current = self.schema_source.load_schema()?;
        require_surreal_provider(&current.datasource.provider)?;
        let previous = self.migration_store.load_last_snapshot()?;

        let name = port
            .migration_name
            .unwrap_or_else(|| format!("auto_{}", Utc::now().format("%Y%m%d%H%M%S")));

        let plan = self.diff.diff(previous.as_ref(), &current, &name);

        if plan.operations.is_empty() {
            return Ok(MigrateDevOutput {
                migration_dir: String::new(),
                operation_count: 0,
                created: false,
            });
        }

        let down_name = format!("{name}_down");
        let down_plan = previous.as_ref().map_or_else(
            || self.diff.diff(Some(&current), &DatabaseSchema::empty(), &down_name),
            |prev| self.diff.diff(Some(&current), prev, &down_name),
        );

        let migration_dir =
            format!("{}/{}_{}", port.migrations_dir, Utc::now().format("%Y%m%d%H%M%S"), name);

        self.filesystem.create_dir_all(&migration_dir)?;

        let surql = self.migration_renderer.render_migration(&plan)?;
        self.filesystem.write_string(&format!("{migration_dir}/migration.surql"), &surql)?;

        let down_surql = self.migration_renderer.render_migration(&down_plan)?;
        self.filesystem
            .write_string(&format!("{migration_dir}/migration.down.surql"), &down_surql)?;

        self.migration_store.save_snapshot(&current, &migration_dir)?;

        Ok(MigrateDevOutput {
            migration_dir,
            operation_count: plan.operations.len(),
            created: true,
        })
    }
}
