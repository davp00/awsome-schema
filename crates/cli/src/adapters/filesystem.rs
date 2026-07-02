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
}
