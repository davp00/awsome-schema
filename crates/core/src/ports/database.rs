use crate::domain::DatabaseConfig;
use crate::errors::DomainError;

pub trait DatabaseExecutor: Send + Sync {
    fn execute_script(&self, config: &DatabaseConfig, script: &str) -> Result<(), DomainError>;
}
