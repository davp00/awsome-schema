use std::sync::Arc;

use parser::parse;
use schema_core::{DatabaseSchema, DomainError, FileSystemPort, SchemaSource};

pub struct SchemaFileSource {
    filesystem: Arc<dyn FileSystemPort>,
    schema_path: String,
}

impl SchemaFileSource {
    pub fn new(filesystem: Arc<dyn FileSystemPort>, schema_path: String) -> Self {
        Self { filesystem, schema_path }
    }
}

impl SchemaSource for SchemaFileSource {
    fn load_schema(&self) -> Result<DatabaseSchema, DomainError> {
        let raw = self.load_raw()?;
        parse(&raw)
    }

    fn load_raw(&self) -> Result<String, DomainError> {
        if !self.filesystem.exists(&self.schema_path) {
            return Err(DomainError::SchemaNotFound(self.schema_path.clone()));
        }
        self.filesystem.read_to_string(&self.schema_path)
    }
}
