use std::path::{Path, PathBuf};

use crate::errors::DomainError;

/// How the running CLI binary appears to have been installed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallKind {
    /// Direct / curl / `cargo install` style binary the user can replace in place.
    Standalone,
    /// Managed by the npm package launcher cache or `node_modules`.
    NpmManaged,
}

/// Locates and replaces the running CLI binary, or reinstalls via crates.io.
pub trait BinaryInstaller: Send + Sync {
    fn current_exe(&self) -> Result<PathBuf, DomainError>;

    fn detect_install_kind(&self, path: &Path) -> InstallKind;

    fn replace_atomic(&self, path: &Path, bytes: &[u8]) -> Result<(), DomainError>;

    fn install_from_crates_io(&self, version: Option<&str>) -> Result<(), DomainError>;
}
