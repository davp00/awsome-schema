use std::sync::Arc;

use crate::domain::{DatabaseConfig, Datasource, require_surreal_provider};
use crate::errors::DomainError;
use crate::ports::{DatabaseExecutor, SchemaRenderer, SchemaSource};

pub struct DbPushInput {
    pub datasource: Datasource,
}

pub struct DbPushOutput {
    pub statements: String,
}

pub struct DbPushUseCase {
    schema_source: Arc<dyn SchemaSource>,
    schema_renderer: Arc<dyn SchemaRenderer>,
    database: Arc<dyn DatabaseExecutor>,
}

impl DbPushUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        schema_renderer: Arc<dyn SchemaRenderer>,
        database: Arc<dyn DatabaseExecutor>,
    ) -> Self {
        Self { schema_source, schema_renderer, database }
    }

    pub fn execute(&self, port: DbPushInput) -> Result<DbPushOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;
        require_surreal_provider(&schema.datasource.provider)?;
        let rendered = self.schema_renderer.render_schema(&schema)?;
        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        self.database.execute_script(&config, &rendered)?;

        Ok(DbPushOutput { statements: rendered })
    }
}
