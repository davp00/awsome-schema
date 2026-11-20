use std::collections::BTreeSet;
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
        // `@relation` nav fields are synthesized from RELATION edges; residual loss is
        // when any preserve edge is missing from the pulled set (by name or table).
        let lossy = edges_lost(&port.preserve, &pulled);
        let schema = merge_pulled_schema(&port.preserve, &pulled);
        Ok(DbPullOutput { schema, lossy })
    }
}

fn edges_lost(preserve: &DatabaseSchema, pulled: &DatabaseSchema) -> bool {
    if preserve.edges.is_empty() {
        return false;
    }
    let pulled_names: BTreeSet<&str> = pulled.edges.iter().map(|edge| edge.name.as_str()).collect();
    let pulled_tables: BTreeSet<String> = pulled
        .edges
        .iter()
        .map(|edge| edge.table_name(&pulled.naming))
        .collect();
    preserve.edges.iter().any(|edge| {
        !pulled_names.contains(edge.name.as_str())
            && !pulled_tables.contains(&edge.table_name(&preserve.naming))
    })
}
