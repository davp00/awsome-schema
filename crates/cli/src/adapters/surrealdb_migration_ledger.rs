use schema_core::{AppliedMigration, DatabaseConfig, DomainError, MigrationLedger};
use serde_json::Value;

use super::surrealdb_connection;

const LEDGER_TABLE: &str = "_awesome_migrations";

const ENSURE_SCHEMA_SCRIPT: &str = r#"
DEFINE TABLE _awesome_migrations SCHEMAFULL;
DEFINE FIELD name ON _awesome_migrations TYPE string;
DEFINE FIELD applied_at ON _awesome_migrations TYPE datetime;
DEFINE FIELD checksum ON _awesome_migrations TYPE string;
DEFINE INDEX _awesome_migrations_name_unique ON _awesome_migrations FIELDS name UNIQUE;
"#;

pub struct SurrealDbMigrationLedger;

impl MigrationLedger for SurrealDbMigrationLedger {
    fn ensure_schema(&self, config: &DatabaseConfig) -> Result<(), DomainError> {
        with_runtime(|runtime| {
            runtime.block_on(async {
                let db = surrealdb_connection::connect(config).await?;
                for statement in split_surql(ENSURE_SCHEMA_SCRIPT) {
                    db.query(statement.as_str())
                        .await
                        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
                }
                Ok(())
            })
        })
    }

    fn list_applied(&self, config: &DatabaseConfig) -> Result<Vec<AppliedMigration>, DomainError> {
        with_runtime(|runtime| {
            runtime.block_on(async {
                let db = surrealdb_connection::connect(config).await?;
                let mut response = db
                    .query(format!(
                        "SELECT name, applied_at, checksum FROM {LEDGER_TABLE} ORDER BY name ASC;"
                    ))
                    .await
                    .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

                let rows: Vec<Value> = response
                    .take(0)
                    .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

                Ok(rows.into_iter().filter_map(parse_ledger_row).collect())
            })
        })
    }

    fn record_applied(
        &self,
        config: &DatabaseConfig,
        name: &str,
        checksum: &str,
    ) -> Result<(), DomainError> {
        with_runtime(|runtime| {
            runtime.block_on(async {
                let db = surrealdb_connection::connect(config).await?;
                let escaped_name = escape_string(name);
                let escaped_checksum = escape_string(checksum);
                let query = format!(
                    "CREATE {LEDGER_TABLE} SET name = '{escaped_name}', applied_at = time::now(), checksum = '{escaped_checksum}';"
                );
                db.query(query)
                    .await
                    .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
                Ok(())
            })
        })
    }

    fn remove_applied(&self, config: &DatabaseConfig, name: &str) -> Result<(), DomainError> {
        with_runtime(|runtime| {
            runtime.block_on(async {
                let db = surrealdb_connection::connect(config).await?;
                let escaped_name = escape_string(name);
                let query = format!("DELETE {LEDGER_TABLE} WHERE name = '{escaped_name}';");
                db.query(query)
                    .await
                    .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
                Ok(())
            })
        })
    }
}

fn parse_ledger_row(value: Value) -> Option<AppliedMigration> {
    let name = value.get("name")?.as_str()?.to_owned();
    let checksum = value
        .get("checksum")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    let applied_at = value.get("applied_at").map(|stamp| match stamp {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    });
    Some(AppliedMigration { name, applied_at, checksum })
}

fn with_runtime<T>(
    f: impl FnOnce(&tokio::runtime::Runtime) -> Result<T, DomainError>,
) -> Result<T, DomainError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    f(&runtime)
}

fn escape_string(value: &str) -> String {
    value.replace('\'', "\\'")
}

fn split_surql(script: &str) -> Vec<String> {
    script
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("--"))
        .map(|line| line.trim_end_matches(';').to_owned())
        .filter(|line| !line.is_empty())
        .map(|line| format!("{line};"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_schema_reports_connection_errors() {
        let config = DatabaseConfig {
            endpoint: "ws://127.0.0.1:1".to_owned(),
            namespace: "test".to_owned(),
            database: "main".to_owned(),
            username: "root".to_owned(),
            password: "root".to_owned(),
        };
        let error = SurrealDbMigrationLedger.ensure_schema(&config).expect_err("connection");
        assert!(matches!(error, DomainError::DatabaseError(_)));
    }
}
