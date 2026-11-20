use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use schema_core::{BinaryInstaller, DomainError, InstallKind};

pub struct FsBinaryInstaller;

impl BinaryInstaller for FsBinaryInstaller {
    fn current_exe(&self) -> Result<PathBuf, DomainError> {
        std::env::current_exe().map_err(|e| DomainError::UpgradeFailed(e.to_string()))
    }

    fn detect_install_kind(&self, path: &Path) -> InstallKind {
        detect_install_kind(path)
    }

    fn replace_atomic(&self, path: &Path, bytes: &[u8]) -> Result<(), DomainError> {
        replace_atomic(path, bytes)
    }

    fn install_from_crates_io(&self, version: Option<&str>) -> Result<(), DomainError> {
        let mut cmd = Command::new("cargo");
        cmd.args(["install", "awesome-schema", "--force"]);
        if let Some(version) = version {
            cmd.args(["--version", version]);
        }
        let status = cmd
            .status()
            .map_err(|e| DomainError::UpgradeFailed(format!("spawn cargo install: {e}")))?;
        if status.success() {
            Ok(())
        } else {
            Err(DomainError::UpgradeFailed(format!(
                "cargo install exited with {status}"
            )))
        }
    }
}

pub fn detect_install_kind(path: &Path) -> InstallKind {
    let text = path.to_string_lossy().replace('\\', "/").to_lowercase();
    if text.contains("/node_modules/")
        || text.contains("/.cache/awesome-schema/")
        || text.contains("/appdata/local/awesome-schema/")
    {
        return InstallKind::NpmManaged;
    }
    InstallKind::Standalone
}

pub fn replace_atomic(path: &Path, bytes: &[u8]) -> Result<(), DomainError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .ok_or_else(|| DomainError::UpgradeFailed("current exe has no file name".into()))?;
    let tmp = parent.join(format!(
        ".{}.upgrade-{}",
        file_name.to_string_lossy(),
        std::process::id()
    ));
    fs::write(&tmp, bytes).map_err(|e| DomainError::UpgradeFailed(e.to_string()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o755))
            .map_err(|e| DomainError::UpgradeFailed(e.to_string()))?;
    }
    // Prefer rename over the target. On Windows a running exe may be locked; fall back to
    // rename-aside then rename.
    match fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(first) => {
            let backup = parent.join(format!(
                ".{}.previous-{}",
                file_name.to_string_lossy(),
                std::process::id()
            ));
            if let Err(e) = fs::rename(path, &backup) {
                let _ = fs::remove_file(&tmp);
                return Err(DomainError::UpgradeFailed(format!(
                    "replace binary failed ({first}); could not move aside old binary ({e})"
                )));
            }
            if let Err(e) = fs::rename(&tmp, path) {
                let _ = fs::rename(&backup, path);
                return Err(DomainError::UpgradeFailed(format!(
                    "replace binary failed after moving aside old binary: {e}"
                )));
            }
            let _ = fs::remove_file(&backup);
            Ok(())
        }
    }
}

/// Rust target triple for published CLI assets, or `None` when unsupported.
#[must_use]
pub fn rust_target() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn detects_npm_cache_paths() {
        assert_eq!(
            detect_install_kind(Path::new("/Users/x/.cache/awesome-schema/0.1.0/bin")),
            InstallKind::NpmManaged
        );
        assert_eq!(
            detect_install_kind(Path::new("/proj/node_modules/.bin/awesome-schema")),
            InstallKind::NpmManaged
        );
        assert_eq!(
            detect_install_kind(Path::new("/usr/local/bin/awesome-schema")),
            InstallKind::Standalone
        );
    }

    #[test]
    fn replace_atomic_writes_executable() {
        let dir = tempdir().expect("temp");
        let dest = dir.path().join("awesome-schema");
        fs::write(&dest, b"old").expect("seed");
        replace_atomic(&dest, b"new-bytes").expect("replace");
        assert_eq!(fs::read(&dest).expect("read"), b"new-bytes");
    }
}
