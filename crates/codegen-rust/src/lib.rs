#![allow(
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::missing_const_for_fn,
    clippy::branches_sharing_code,
    clippy::option_if_let_else
)]

mod emit;

use codegen::CodeGenerator;
use core::usecases::CodeGeneratorPort;
use core::{DatabaseSchema, DomainError};

pub struct RustGenerator;

impl RustGenerator {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator for RustGenerator {
    fn language(&self) -> &'static str {
        "rust"
    }

    fn generate(&self, schema: &DatabaseSchema) -> Result<String, DomainError> {
        if schema.datasource.provider != "surrealdb" {
            return Err(DomainError::CodegenError(format!(
                "rust client for provider `{}` is not implemented; only surrealdb is supported",
                schema.datasource.provider
            )));
        }
        Ok(emit::emit_rust(schema))
    }
}

impl CodeGeneratorPort for RustGenerator {
    fn generate(&self, schema: &DatabaseSchema) -> Result<String, DomainError> {
        CodeGenerator::generate(self, schema)
    }
}
