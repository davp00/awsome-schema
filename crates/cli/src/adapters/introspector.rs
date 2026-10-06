use std::collections::BTreeMap;

use renderers::{map_database_info, TableInfo};
use schema_core::{DatabaseConfig, DatabaseSchema, DomainError, SchemaIntrospector};
use serde_json::Value;

use super::surrealdb_connection;

pub struct SurrealDbIntrospector;

impl SchemaIntrospector for SurrealDbIntrospector {
    fn introspect(
        &self,
        config: &DatabaseConfig,
        preserve: &DatabaseSchema,
    ) -> Result<DatabaseSchema, DomainError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

        runtime.block_on(introspect_async(config, preserve))
    }
}

async fn introspect_async(
    config: &DatabaseConfig,
    preserve: &DatabaseSchema,
) -> Result<DatabaseSchema, DomainError> {
    let db = surrealdb_connection::connect(config).await?;

    let mut response = db
        .query("INFO FOR DB;")
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    let info: Option<Value> = response
        .take(0)
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    let Some(info) = info else {
        return Ok(DatabaseSchema::empty());
    };

    let tables = info
        .get("tables")
        .and_then(Value::as_object)
        .ok_or_else(|| DomainError::DatabaseError("INFO FOR DB missing tables".into()))?;

    let mut table_defines = BTreeMap::new();
    for (name, value) in tables {
        let define = value.as_str().unwrap_or("").to_owned();
        table_defines.insert(name.clone(), define);
    }

    let mut table_infos = BTreeMap::new();
    for table in table_defines.keys() {
        let query = format!("INFO FOR TABLE `{table}`;");
        let mut table_response = db
            .query(query)
            .await
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
        let table_info: Option<Value> = table_response
            .take(0)
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

        let mut mapped = TableInfo::default();
        if let Some(value) = table_info {
            if let Some(fields) = value.get("fields").and_then(Value::as_object) {
                for (name, define) in fields {
                    if let Some(text) = define.as_str() {
                        mapped.fields.insert(name.clone(), text.to_owned());
                    }
                }
            }
            if let Some(indexes) = value.get("indexes").and_then(Value::as_object) {
                for (name, define) in indexes {
                    if let Some(text) = define.as_str() {
                        mapped.indexes.insert(name.clone(), text.to_owned());
                    }
                }
            }
        }
        table_infos.insert(table.clone(), mapped);
    }

    map_database_info(&table_defines, &table_infos, preserve)
}
