use crate::errors::DomainError;

/// Fetches Awesome Schema release metadata and assets (typically GitHub Releases).
pub trait ReleaseClient: Send + Sync {
    /// Latest stable release tag (with or without a leading `v`).
    fn latest_stable_tag(&self) -> Result<String, DomainError>;

    /// Download the CLI binary asset for `tag` and Rust target triple.
    fn download_asset(&self, tag: &str, target: &str) -> Result<Vec<u8>, DomainError>;
}
