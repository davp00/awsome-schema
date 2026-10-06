use schema_core::{DomainError, FileSystemPort};

pub struct FsAdapter;

impl FileSystemPort for FsAdapter {
    fn read_to_string(&self, path: &str) -> Result<String, DomainError> {
        std::fs::read_to_string(path)
            .map_err(|error| DomainError::SchemaReadFailed(error.to_string()))
    }

    fn write_string(&self, path: &str, content: &str) -> Result<(), DomainError> {
        std::fs::write(path, content).map_err(|error| DomainError::WriteFailed(error.to_string()))
    }

    fn create_dir_all(&self, path: &str) -> Result<(), DomainError> {
        std::fs::create_dir_all(path).map_err(|error| DomainError::WriteFailed(error.to_string()))
    }

    fn exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn is_directory(&self, path: &str) -> bool {
        std::path::Path::new(path).is_dir()
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>, DomainError> {
        let mut names = Vec::new();
        for entry in std::fs::read_dir(path).map_err(|error| DomainError::SchemaReadFailed(error.to_string()))? {
            let entry = entry.map_err(|error| DomainError::SchemaReadFailed(error.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_owned());
            }
        }
        names.sort();
        Ok(names)
    }

    fn remove_file(&self, path: &str) -> Result<(), DomainError> {
        std::fs::remove_file(path).map_err(|error| DomainError::WriteFailed(error.to_string()))
    }
}
