use schema_core::{DatabaseConfig, DatabaseExecutor, DomainError, ws_connection_address};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

pub struct SurrealDbExecutor;

impl DatabaseExecutor for SurrealDbExecutor {
    fn execute_script(&self, config: &DatabaseConfig, script: &str) -> Result<(), DomainError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

        runtime.block_on(execute_script_async(config, script))
    }
}

async fn execute_script_async(config: &DatabaseConfig, script: &str) -> Result<(), DomainError> {
    let address = ws_connection_address(&config.endpoint);
    let db = Surreal::new::<Ws>(address.to_owned())
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    db.signin(Root { username: &config.username, password: &config.password })
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    db.use_ns(&config.namespace)
        .use_db(&config.database)
        .await
        .map_err(|error| DomainError::DatabaseError(error.to_string()))?;

    for statement in split_surql(script) {
        db.query(statement.as_str())
            .await
            .map_err(|error| DomainError::DatabaseError(error.to_string()))?;
    }

    Ok(())
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
