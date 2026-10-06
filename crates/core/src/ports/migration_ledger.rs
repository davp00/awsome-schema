use crate::domain::DatabaseConfig;
use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedMigration {
    pub name: String,
    pub applied_at: Option<String>,
    pub checksum: String,
}

pub trait MigrationLedger: Send + Sync {
    fn ensure_schema(&self, config: &DatabaseConfig) -> Result<(), DomainError>;

    fn list_applied(&self, config: &DatabaseConfig) -> Result<Vec<AppliedMigration>, DomainError>;

    fn record_applied(
        &self,
        config: &DatabaseConfig,
        name: &str,
        checksum: &str,
    ) -> Result<(), DomainError>;

    fn remove_applied(&self, config: &DatabaseConfig, name: &str) -> Result<(), DomainError>;
}
