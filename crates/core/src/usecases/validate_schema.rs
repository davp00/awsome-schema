use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::SchemaSource;
use crate::validation::validate_schema;

pub struct ValidateSchemaInput;

pub struct ValidateSchemaOutput {
    pub model_count: usize,
    pub edge_count: usize,
}

pub struct ValidateSchemaUseCase {
    schema_source: Arc<dyn SchemaSource>,
}

impl ValidateSchemaUseCase {
    pub fn new(schema_source: Arc<dyn SchemaSource>) -> Self {
        Self { schema_source }
    }

    pub fn execute(&self, _port: ValidateSchemaInput) -> Result<ValidateSchemaOutput, DomainError> {
        let schema = self.schema_source.load_schema()?;
        validate_schema(&schema)?;

        Ok(ValidateSchemaOutput {
            model_count: schema.models.len(),
            edge_count: schema.edges.len(),
        })
    }
}
