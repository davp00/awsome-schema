#![allow(clippy::missing_errors_doc)]

mod surreal;
mod types;

use codegen::CodeGenerator;
use core::usecases::CodeGeneratorPort;
use core::{DatabaseSchema, DomainError};
use surreal::SurrealTypeScriptProvider;

pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for TypeScriptGenerator {
    fn default() -> Self {
        Self::new()
    }
}

trait TypeScriptProvider {
    fn id(&self) -> &'static str;
    fn emit(&self, schema: &DatabaseSchema) -> Result<String, DomainError>;
}

static SURREAL: SurrealTypeScriptProvider = SurrealTypeScriptProvider;

fn typescript_provider(provider: &str) -> Result<&'static dyn TypeScriptProvider, DomainError> {
    if provider == SURREAL.id() {
        return Ok(&SURREAL);
    }
    Err(DomainError::CodegenError(format!(
        "typescript client for provider `{provider}` is not implemented; only surrealdb is supported"
    )))
}

impl CodeGenerator for TypeScriptGenerator {
    fn language(&self) -> &'static str {
        "typescript"
    }

    fn generate(&self, schema: &DatabaseSchema) -> Result<String, DomainError> {
        typescript_provider(&schema.datasource.provider)?.emit(schema)
    }
}

impl CodeGeneratorPort for TypeScriptGenerator {
    fn generate(&self, schema: &DatabaseSchema) -> Result<String, DomainError> {
        CodeGenerator::generate(self, schema)
    }
}
