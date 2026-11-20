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
        let config = DatabaseConfig::from_datasource(&port.datasource)?;
        ensure_fulltext_analyzers_exist(self.database.as_ref(), &config, &schema)?;
        let rendered = self.schema_renderer.render_schema(&schema)?;
        self.database.execute_script(&config, &rendered)?;

        Ok(DbPushOutput { statements: rendered })
    }
}

fn ensure_fulltext_analyzers_exist(
    database: &dyn DatabaseExecutor,
    config: &DatabaseConfig,
    schema: &crate::domain::DatabaseSchema,
) -> Result<(), DomainError> {
    let mut needed: Vec<String> = schema
        .models
        .iter()
        .flat_map(|model| model.indexes.iter())
        .filter(|index| index.fulltext)
        .filter_map(|index| index.fulltext_analyzer.clone())
        .collect();
    needed.sort();
    needed.dedup();
    if needed.is_empty() {
        return Ok(());
    }

    let available = database.list_analyzers(config)?;
    for name in needed {
        if !available.iter().any(|analyzer| analyzer == &name) {
            return Err(DomainError::ValidationError(format!(
                "fulltext analyzer `{name}` is not defined in the database; \
                 DEFINE ANALYZER `{name}` … before `db push` (see INFO FOR DB analyzers)"
            )));
        }
    }
    Ok(())
}
