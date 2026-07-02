use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::{SchemaRenderer, SchemaSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateCodeTarget {
    Schema,
    Rust,
    TypeScript,
}

pub struct GenerateCodeInput {
    pub target: GenerateCodeTarget,
}

pub struct GenerateCodeOutput {
    pub content: String,
    pub target: GenerateCodeTarget,
}

pub struct GenerateCodeUseCase {
    schema_source: Arc<dyn SchemaSource>,
    schema_renderer: Arc<dyn SchemaRenderer>,
    rust_generator: Arc<dyn CodeGeneratorPort>,
    typescript_generator: Arc<dyn CodeGeneratorPort>,
}

pub trait CodeGeneratorPort: Send + Sync {
    fn generate(&self, schema: &crate::domain::DatabaseSchema) -> Result<String, DomainError>;
}

impl GenerateCodeUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        schema_renderer: Arc<dyn SchemaRenderer>,
        rust_generator: Arc<dyn CodeGeneratorPort>,
        typescript_generator: Arc<dyn CodeGeneratorPort>,
    ) -> Self {
        Self { schema_source, schema_renderer, rust_generator, typescript_generator }
    }

    pub fn execute(&self, port: GenerateCodeInput) -> Result<GenerateCodeOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;

        let content = match port.target {
            GenerateCodeTarget::Schema => self.schema_renderer.render_schema(&schema)?,
            GenerateCodeTarget::Rust => self.rust_generator.generate(&schema)?,
            GenerateCodeTarget::TypeScript => self.typescript_generator.generate(&schema)?,
        };

        Ok(GenerateCodeOutput { content, target: port.target })
    }
}
