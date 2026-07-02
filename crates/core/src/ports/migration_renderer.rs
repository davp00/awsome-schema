use crate::domain::MigrationPlan;
use crate::errors::DomainError;

pub trait MigrationRenderer: Send + Sync {
    fn render_migration(&self, migration: &MigrationPlan) -> Result<String, DomainError>;
}
