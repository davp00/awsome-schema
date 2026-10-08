use std::sync::Arc;

use crate::domain::{DatabaseConfig, DatabaseSchema, merge_pulled_schema};
use crate::errors::DomainError;
use crate::ports::SchemaIntrospector;

pub struct DbPullInput {
    pub preserve: DatabaseSchema,
}

pub struct DbPullOutput {
    pub schema: DatabaseSchema,
    pub lossy: bool,
}

pub struct DbPullUseCase {
    introspector: Arc<dyn SchemaIntrospector>,
}

impl DbPullUseCase {
    pub fn new(introspector: Arc<dyn SchemaIntrospector>) -> Self {
        Self { introspector }
    }

    pub fn execute(&self, port: DbPullInput) -> Result<DbPullOutput, DomainError> {
        let config = DatabaseConfig::from_datasource(&port.preserve.datasource)?;
        let pulled = self.introspector.introspect(&config, &port.preserve)?;
        let lossy = pulled
            .models
            .iter()
            .any(|model| model.fields.iter().any(|field| field.relation_name.is_some()))
            || !port.preserve.edges.is_empty() && pulled.edges.is_empty();
        let schema = merge_pulled_schema(&port.preserve, &pulled);
        Ok(DbPullOutput { schema, lossy })
    }
}
