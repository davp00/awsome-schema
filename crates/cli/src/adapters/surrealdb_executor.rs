use schema_core::{DatabaseConfig, DatabaseExecutor, DomainError};
use serde_json::Value;

use super::surrealdb_connection;

pub struct SurrealDbExecutor;

impl DatabaseExecutor for SurrealDbExecutor {
    fn execute_script(&self, config: &DatabaseConfig, script: &str) -> Result<(), DomainError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

        runtime.block_on(execute_script_async(config, script))
    }

    fn list_analyzers(&self, config: &DatabaseConfig) -> Result<Vec<String>, DomainError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

        runtime.block_on(list_analyzers_async(config))
    }
}

async fn execute_script_async(config: &DatabaseConfig, script: &str) -> Result<(), DomainError> {
    let db = surrealdb_connection::connect(config).await?;

    for statement in split_surql(script) {
        db.query(statement.as_str())
            .await
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    }

    Ok(())
}

async fn list_analyzers_async(config: &DatabaseConfig) -> Result<Vec<String>, DomainError> {
    let db = surrealdb_connection::connect(config).await?;
    let mut response = db
        .query("INFO FOR DB;")
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    let info: Option<Value> = response
        .take(0)
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    let Some(info) = info else {
        return Ok(Vec::new());
    };
    let Some(analyzers) = info.get("analyzers").and_then(Value::as_object) else {
        return Ok(Vec::new());
    };
    Ok(analyzers.keys().cloned().collect())
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
    fn execute_script_reports_connection_errors() {
        let config = DatabaseConfig {
            endpoint: "ws://127.0.0.1:1".to_owned(),
            namespace: "test".to_owned(),
            database: "main".to_owned(),
            username: "root".to_owned(),
            password: "root".to_owned(),
        };
        let error = SurrealDbExecutor
            .execute_script(&config, "DEFINE TABLE user SCHEMAFULL;")
            .expect_err("connection");
        assert!(matches!(error, DomainError::DatabaseError(_)));
    }
}
