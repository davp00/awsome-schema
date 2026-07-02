use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::FileSystemPort;
use crate::templates::DEFAULT_SCHEMA_TEMPLATE;

pub struct InitProjectInput {
    pub schema_path: String,
    pub migrations_dir: String,
}

pub struct InitProjectOutput {
    pub schema_created: bool,
    pub migrations_dir_created: bool,
}

pub struct InitProjectUseCase {
    filesystem: Arc<dyn FileSystemPort>,
}

impl InitProjectUseCase {
    pub fn new(filesystem: Arc<dyn FileSystemPort>) -> Self {
        Self { filesystem }
    }

    pub fn execute(&self, port: InitProjectInput) -> Result<InitProjectOutput, DomainError> {
        self.filesystem.create_dir_all(&port.migrations_dir)?;

        let schema_created = if self.filesystem.exists(&port.schema_path) {
            false
        } else {
            self.filesystem.write_string(&port.schema_path, DEFAULT_SCHEMA_TEMPLATE)?;
            true
        };

        Ok(InitProjectOutput { schema_created, migrations_dir_created: true })
    }
}
