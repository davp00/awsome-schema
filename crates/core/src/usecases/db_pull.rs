use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::SchemaIntrospector;

pub struct DbPullInput;

pub struct DbPullOutput {
    pub schema: crate::domain::DatabaseSchema,
}

pub struct DbPullUseCase {
    introspector: Arc<dyn SchemaIntrospector>,
}

impl DbPullUseCase {
    pub fn new(introspector: Arc<dyn SchemaIntrospector>) -> Self {
        Self { introspector }
    }

    pub fn execute(&self, _port: DbPullInput) -> Result<DbPullOutput, DomainError> {
        let schema = self.introspector.introspect()?;
        Ok(DbPullOutput { schema })
    }
}
