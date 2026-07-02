use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::{FileSystemPort, SchemaSource};

pub struct FormatSchemaInput {
    pub write_back: bool,
}

pub struct FormatSchemaOutput {
    pub formatted: String,
    pub written: bool,
}

pub struct FormatSchemaUseCase {
    schema_source: Arc<dyn SchemaSource>,
    filesystem: Arc<dyn FileSystemPort>,
    schema_path: String,
}

impl FormatSchemaUseCase {
    pub fn new(
        schema_source: Arc<dyn SchemaSource>,
        filesystem: Arc<dyn FileSystemPort>,
        schema_path: String,
    ) -> Self {
        Self { schema_source, filesystem, schema_path }
    }

    pub fn execute(&self, port: FormatSchemaInput) -> Result<FormatSchemaOutput, DomainError> {
        let _schema = self.schema_source.load_schema()?;
        let raw = self.schema_source.load_raw()?;
        let formatted = normalize_whitespace(&raw);

        let written = if port.write_back {
            self.filesystem.write_string(&self.schema_path, &formatted)?;
            true
        } else {
            false
        };

        Ok(FormatSchemaOutput { formatted, written })
    }
}

fn normalize_whitespace(input: &str) -> String {
    let mut lines = input.lines().map(str::trim_end).collect::<Vec<_>>();

    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }

    if lines.is_empty() { String::new() } else { format!("{}\n", lines.join("\n")) }
}
