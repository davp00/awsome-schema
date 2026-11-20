use crate::domain::DatabaseConfig;
use crate::errors::DomainError;

pub trait DatabaseExecutor: Send + Sync {
    fn execute_script(&self, config: &DatabaseConfig, script: &str) -> Result<(), DomainError>;

    /// Analyzer names from `INFO FOR DB` (`analyzers` map keys).
    fn list_analyzers(&self, config: &DatabaseConfig) -> Result<Vec<String>, DomainError>;
}
