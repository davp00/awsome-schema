use crate::errors::DomainError;

pub trait FileSystemPort: Send + Sync {
    fn read_to_string(&self, path: &str) -> Result<String, DomainError>;
    fn write_string(&self, path: &str, content: &str) -> Result<(), DomainError>;
    fn create_dir_all(&self, path: &str) -> Result<(), DomainError>;
    fn exists(&self, path: &str) -> bool;
    fn is_directory(&self, path: &str) -> bool;
    fn list_dir(&self, path: &str) -> Result<Vec<String>, DomainError>;
    fn remove_file(&self, path: &str) -> Result<(), DomainError>;
}
