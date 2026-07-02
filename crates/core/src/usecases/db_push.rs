use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::{MigrationRenderer, SchemaRenderer, SchemaSource};

pub struct DbPushInput;

pub struct DbPushOutput {
    pub statements: String,
}

pub struct DbPushUseCase {
    schema_source: Arc<dyn SchemaSource>,
    schema_renderer: Arc<dyn SchemaRenderer>,
    migration_renderer: Arc<dyn MigrationRenderer>,
}

impl DbPushUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        schema_renderer: Arc<dyn SchemaRenderer>,
        migration_renderer: Arc<dyn MigrationRenderer>,
    ) -> Self {
        Self { schema_source, schema_renderer, migration_renderer }
    }

    pub fn execute(&self, _port: DbPushInput) -> Result<DbPushOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;
        let rendered = self.schema_renderer.render_schema(&schema)?;
        let _migration =
            self.migration_renderer.render_migration(&crate::domain::MigrationPlan {
                name: "push".to_owned(),
                operations: Vec::new(),
            })?;

        Err(DomainError::NotImplemented(format!(
            "db push will apply rendered schema to the database (preview ready, {rendered} bytes)"
        )))
    }
}
