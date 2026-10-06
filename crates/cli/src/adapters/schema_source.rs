use std::sync::Arc;

use parser::parse;
use schema_core::{DatabaseSchema, DomainError, FileSystemPort, SchemaSource};

use crate::schema_layout::{CONFIG_FILE, TABLES_DIR, TABLE_SUFFIX};

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
        if self.filesystem.is_directory(&self.schema_path) {
            load_directory_schema(&*self.filesystem, &self.schema_path)
        } else {
            parse(&self.load_raw()?)
        }
    }

    fn load_raw(&self) -> Result<String, DomainError> {
        if self.filesystem.is_directory(&self.schema_path) {
            return Err(DomainError::SchemaReadFailed(
                "load_raw requires a single schema file path".into(),
            ));
        }
        if !self.filesystem.exists(&self.schema_path) {
            return Err(DomainError::SchemaNotFound(self.schema_path.clone()));
        }
        self.filesystem.read_to_string(&self.schema_path)
    }
}

fn load_directory_schema(
    filesystem: &dyn FileSystemPort,
    dir: &str,
) -> Result<DatabaseSchema, DomainError> {
    let config_path = format!("{dir}/{CONFIG_FILE}");
    if !filesystem.exists(&config_path) {
        return Err(DomainError::SchemaNotFound(config_path));
    }
    let mut schema = parse(&filesystem.read_to_string(&config_path)?)?;

    let tables_dir = format!("{dir}/{TABLES_DIR}");
    if filesystem.is_directory(&tables_dir) {
        for name in filesystem.list_dir(&tables_dir)? {
            if !name.ends_with(TABLE_SUFFIX) {
                continue;
            }
            let fragment_path = format!("{tables_dir}/{name}");
            let fragment = parse(&filesystem.read_to_string(&fragment_path)?)?;
            schema.models.extend(fragment.models);
            schema.edges.extend(fragment.edges);
        }
    }

    Ok(schema)
}

pub fn list_schema_files(filesystem: &dyn FileSystemPort, schema_path: &str) -> Result<Vec<String>, DomainError> {
    if filesystem.is_directory(schema_path) {
        let mut paths = vec![format!("{schema_path}/{CONFIG_FILE}")];
        let tables_dir = format!("{schema_path}/{TABLES_DIR}");
        if filesystem.is_directory(&tables_dir) {
            for name in filesystem.list_dir(&tables_dir)? {
                if name.ends_with(TABLE_SUFFIX) {
                    paths.push(format!("{tables_dir}/{name}"));
                }
            }
        }
        Ok(paths)
    } else {
        Ok(vec![schema_path.to_owned()])
    }
}
