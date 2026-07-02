use crate::errors::DomainError;

pub trait FileSystemPort: Send + Sync {
    fn read_to_string(&self, path: &str) -> Result<String, DomainError>;
    fn write_string(&self, path: &str, content: &str) -> Result<(), DomainError>;
    fn create_dir_all(&self, path: &str) -> Result<(), DomainError>;
    fn exists(&self, path: &str) -> bool;
}
