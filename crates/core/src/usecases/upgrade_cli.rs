use std::sync::Arc;

use crate::errors::DomainError;
use crate::ports::{BinaryInstaller, InstallKind, ReleaseClient};

/// Where to fetch the upgraded CLI from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeSource {
    Github,
    CratesIo,
}

pub struct UpgradeCliInput {
    pub current_version: String,
    pub check_only: bool,
    pub version: Option<String>,
    pub source: UpgradeSource,
    /// Rust target triple for this machine (e.g. `aarch64-apple-darwin`).
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeCliOutput {
    pub current_version: String,
    pub latest_version: String,
    pub updated: bool,
    pub message: String,
}

pub struct UpgradeCliUseCase {
    releases: Arc<dyn ReleaseClient>,
    installer: Arc<dyn BinaryInstaller>,
}

impl UpgradeCliUseCase {
    pub fn new(releases: Arc<dyn ReleaseClient>, installer: Arc<dyn BinaryInstaller>) -> Self {
        Self { releases, installer }
    }

    pub fn execute(&self, input: UpgradeCliInput) -> Result<UpgradeCliOutput, DomainError> {
        if input.target.trim().is_empty() {
            return Err(DomainError::UnsupportedPlatform(
                "no awesome-schema binary for this platform".into(),
            ));
        }

        let exe = self.installer.current_exe()?;
        if self.installer.detect_install_kind(&exe) == InstallKind::NpmManaged {
            return Err(DomainError::NpmManagedInstall(
                "this binary is managed by the npm package; run `pnpm update awesome-schema` or `npm update awesome-schema` instead".into(),
            ));
        }

        match input.source {
            UpgradeSource::CratesIo => self.upgrade_crates_io(input),
            UpgradeSource::Github => self.upgrade_github(input, &exe),
        }
    }

    fn upgrade_crates_io(&self, input: UpgradeCliInput) -> Result<UpgradeCliOutput, DomainError> {
        let latest = input
            .version
            .clone()
            .unwrap_or_else(|| input.current_version.clone());
        if input.check_only {
            return Ok(UpgradeCliOutput {
                current_version: input.current_version.clone(),
                latest_version: latest,
                updated: false,
                message: format!(
                    "crates.io upgrade check: reinstall with `cargo install awesome-schema --force`{}",
                    input
                        .version
                        .as_ref()
                        .map(|v| format!(" --version {v}"))
                        .unwrap_or_default()
                ),
            });
        }
        self.installer
            .install_from_crates_io(input.version.as_deref())?;
        let latest = input
            .version
            .clone()
            .unwrap_or_else(|| input.current_version.clone());
        Ok(UpgradeCliOutput {
            current_version: input.current_version,
            latest_version: latest,
            updated: true,
            message: "reinstalled awesome-schema from crates.io".into(),
        })
    }

    fn upgrade_github(
        &self,
        input: UpgradeCliInput,
        exe: &std::path::Path,
    ) -> Result<UpgradeCliOutput, DomainError> {
        let tag = match &input.version {
            Some(v) => normalize_tag(v),
            None => normalize_tag(&self.releases.latest_stable_tag()?),
        };
        let latest = strip_v(&tag);

        if versions_equal(&input.current_version, &latest) {
            return Ok(UpgradeCliOutput {
                current_version: input.current_version,
                latest_version: latest,
                updated: false,
                message: "already up to date".into(),
            });
        }

        if input.check_only {
            return Ok(UpgradeCliOutput {
                current_version: input.current_version,
                latest_version: latest.clone(),
                updated: false,
                message: format!("update available: {latest}"),
            });
        }

        let bytes = self.releases.download_asset(&tag, &input.target)?;
        self.installer.replace_atomic(exe, &bytes)?;

        Ok(UpgradeCliOutput {
            current_version: input.current_version,
            latest_version: latest.clone(),
            updated: true,
            message: format!("upgraded to {latest}"),
        })
    }
}

fn normalize_tag(version: &str) -> String {
    let trimmed = version.trim();
    if trimmed.starts_with('v') {
        trimmed.to_owned()
    } else {
        format!("v{trimmed}")
    }
}

fn strip_v(tag: &str) -> String {
    tag.trim().trim_start_matches('v').to_owned()
}

fn versions_equal(a: &str, b: &str) -> bool {
    strip_v(a) == strip_v(b)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    use super::*;

    struct FakeRelease {
        latest: String,
        asset: Vec<u8>,
        fail_download: bool,
    }

    impl ReleaseClient for FakeRelease {
        fn latest_stable_tag(&self) -> Result<String, DomainError> {
            Ok(self.latest.clone())
        }

        fn download_asset(&self, _tag: &str, _target: &str) -> Result<Vec<u8>, DomainError> {
            if self.fail_download {
                return Err(DomainError::DownloadFailed("boom".into()));
            }
            Ok(self.asset.clone())
        }
    }

    struct FakeInstaller {
        exe: PathBuf,
        kind: InstallKind,
        replaced: Mutex<Option<Vec<u8>>>,
        crates_io_calls: Mutex<u32>,
    }

    impl BinaryInstaller for FakeInstaller {
        fn current_exe(&self) -> Result<PathBuf, DomainError> {
            Ok(self.exe.clone())
        }

        fn detect_install_kind(&self, _path: &Path) -> InstallKind {
            self.kind
        }

        fn replace_atomic(&self, _path: &Path, bytes: &[u8]) -> Result<(), DomainError> {
            *self.replaced.lock().expect("lock") = Some(bytes.to_vec());
            Ok(())
        }

        fn install_from_crates_io(&self, _version: Option<&str>) -> Result<(), DomainError> {
            *self.crates_io_calls.lock().expect("lock") += 1;
            Ok(())
        }
    }

    struct SharedInstaller {
        exe: PathBuf,
        kind: InstallKind,
        replaced: Arc<Mutex<Option<Vec<u8>>>>,
    }

    impl BinaryInstaller for SharedInstaller {
        fn current_exe(&self) -> Result<PathBuf, DomainError> {
            Ok(self.exe.clone())
        }

        fn detect_install_kind(&self, _: &Path) -> InstallKind {
            self.kind
        }

        fn replace_atomic(&self, _: &Path, bytes: &[u8]) -> Result<(), DomainError> {
            *self.replaced.lock().expect("lock") = Some(bytes.to_vec());
            Ok(())
        }

        fn install_from_crates_io(&self, _: Option<&str>) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct CratesInstaller {
        calls: Arc<Mutex<u32>>,
    }

    impl BinaryInstaller for CratesInstaller {
        fn current_exe(&self) -> Result<PathBuf, DomainError> {
            Ok(PathBuf::from("/home/x/.cargo/bin/awesome-schema"))
        }

        fn detect_install_kind(&self, _: &Path) -> InstallKind {
            InstallKind::Standalone
        }

        fn replace_atomic(&self, _: &Path, _: &[u8]) -> Result<(), DomainError> {
            Err(DomainError::NotImplemented("no".into()))
        }

        fn install_from_crates_io(&self, _: Option<&str>) -> Result<(), DomainError> {
            *self.calls.lock().expect("lock") += 1;
            Ok(())
        }
    }

    fn usecase(releases: FakeRelease, installer: FakeInstaller) -> UpgradeCliUseCase {
        UpgradeCliUseCase::new(Arc::new(releases), Arc::new(installer))
    }

    #[test]
    fn check_reports_available_update() {
        let uc = usecase(
            FakeRelease {
                latest: "v0.2.0".into(),
                asset: vec![1, 2, 3],
                fail_download: false,
            },
            FakeInstaller {
                exe: PathBuf::from("/usr/local/bin/awesome-schema"),
                kind: InstallKind::Standalone,
                replaced: Mutex::new(None),
                crates_io_calls: Mutex::new(0),
            },
        );
        let out = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: true,
                version: None,
                source: UpgradeSource::Github,
                target: "aarch64-apple-darwin".into(),
            })
            .expect("ok");
        assert!(!out.updated);
        assert_eq!(out.latest_version, "0.2.0");
        assert!(out.message.contains("update available"));
    }

    #[test]
    fn github_upgrade_replaces_binary() {
        let replaced_bytes = Arc::new(Mutex::new(None));
        let uc = UpgradeCliUseCase::new(
            Arc::new(FakeRelease {
                latest: "v0.2.0".into(),
                asset: vec![9, 9, 9],
                fail_download: false,
            }),
            Arc::new(SharedInstaller {
                exe: PathBuf::from("/bin/awesome-schema"),
                kind: InstallKind::Standalone,
                replaced: replaced_bytes.clone(),
            }),
        );
        let out = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: false,
                version: None,
                source: UpgradeSource::Github,
                target: "x86_64-unknown-linux-gnu".into(),
            })
            .expect("ok");
        assert!(out.updated);
        assert_eq!(out.latest_version, "0.2.0");
        assert_eq!(
            replaced_bytes.lock().expect("lock").as_ref().expect("bytes"),
            &vec![9, 9, 9]
        );
    }

    #[test]
    fn already_up_to_date_skips_download() {
        let uc = usecase(
            FakeRelease {
                latest: "v0.1.0-alpha.1".into(),
                asset: vec![1],
                fail_download: true,
            },
            FakeInstaller {
                exe: PathBuf::from("/bin/awesome-schema"),
                kind: InstallKind::Standalone,
                replaced: Mutex::new(None),
                crates_io_calls: Mutex::new(0),
            },
        );
        let out = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: false,
                version: None,
                source: UpgradeSource::Github,
                target: "aarch64-apple-darwin".into(),
            })
            .expect("ok");
        assert!(!out.updated);
        assert_eq!(out.message, "already up to date");
    }

    #[test]
    fn npm_managed_is_rejected() {
        let uc = usecase(
            FakeRelease {
                latest: "v0.2.0".into(),
                asset: vec![],
                fail_download: false,
            },
            FakeInstaller {
                exe: PathBuf::from("/Users/x/.cache/awesome-schema/0.1.0/bin"),
                kind: InstallKind::NpmManaged,
                replaced: Mutex::new(None),
                crates_io_calls: Mutex::new(0),
            },
        );
        let err = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: false,
                version: None,
                source: UpgradeSource::Github,
                target: "aarch64-apple-darwin".into(),
            })
            .expect_err("npm");
        assert_eq!(err.code(), "NPM_MANAGED_INSTALL");
    }

    #[test]
    fn crates_io_source_invokes_installer() {
        let calls = Arc::new(Mutex::new(0_u32));
        let uc = UpgradeCliUseCase::new(
            Arc::new(FakeRelease {
                latest: "v0.2.0".into(),
                asset: vec![],
                fail_download: false,
            }),
            Arc::new(CratesInstaller {
                calls: calls.clone(),
            }),
        );
        let out = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: false,
                version: Some("0.1.0-alpha.1".into()),
                source: UpgradeSource::CratesIo,
                target: "aarch64-apple-darwin".into(),
            })
            .expect("ok");
        assert!(out.updated);
        assert_eq!(*calls.lock().expect("lock"), 1);
    }

    #[test]
    fn empty_target_is_unsupported() {
        let uc = usecase(
            FakeRelease {
                latest: "v0.2.0".into(),
                asset: vec![],
                fail_download: false,
            },
            FakeInstaller {
                exe: PathBuf::from("/bin/awesome-schema"),
                kind: InstallKind::Standalone,
                replaced: Mutex::new(None),
                crates_io_calls: Mutex::new(0),
            },
        );
        let err = uc
            .execute(UpgradeCliInput {
                current_version: "0.1.0-alpha.1".into(),
                check_only: true,
                version: None,
                source: UpgradeSource::Github,
                target: String::new(),
            })
            .expect_err("platform");
        assert_eq!(err.code(), "UNSUPPORTED_PLATFORM");
    }
}
