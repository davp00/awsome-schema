#![allow(clippy::missing_errors_doc)]

use core::{DatabaseSchema, DomainError};

pub trait CodeGenerator: Send + Sync {
    fn language(&self) -> &'static str;
    fn generate(&self, schema: &DatabaseSchema) -> Result<String, DomainError>;
}
