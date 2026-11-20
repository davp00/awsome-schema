use schema_core::{DomainError, ReleaseClient};
use serde_json::Value;

const RELEASE_REPO: &str = "davp00/awsome-schema";

pub struct GithubReleaseClient;

impl GithubReleaseClient {
    fn asset_name(target: &str) -> String {
        let ext = if target.contains("windows") { ".exe" } else { "" };
        format!("awesome-schema-{target}{ext}")
    }

    fn release_url(tag: &str, target: &str) -> String {
        let tag = if tag.starts_with('v') {
            tag.to_owned()
        } else {
            format!("v{tag}")
        };
        format!(
            "https://github.com/{RELEASE_REPO}/releases/download/{tag}/{}",
            Self::asset_name(target)
        )
    }
}

impl ReleaseClient for GithubReleaseClient {
    fn latest_stable_tag(&self) -> Result<String, DomainError> {
        let url = format!("https://api.github.com/repos/{RELEASE_REPO}/releases");
        let body = http_get(&url)?;
        let releases: Value = serde_json::from_str(&body)
            .map_err(|e| DomainError::DownloadFailed(format!("parse releases: {e}")))?;
        let Some(arr) = releases.as_array() else {
            return Err(DomainError::DownloadFailed("releases response was not an array".into()));
        };
        for release in arr {
            if release.get("draft").and_then(Value::as_bool) == Some(true) {
                continue;
            }
            let Some(tag) = release.get("tag_name").and_then(Value::as_str) else {
                continue;
            };
            if !tag.starts_with('v') {
                continue;
            }
            if tag.contains("canary") || tag.contains("test") {
                continue;
            }
            return Ok(tag.to_owned());
        }
        Err(DomainError::DownloadFailed("no stable GitHub release tag found".into()))
    }

    fn download_asset(&self, tag: &str, target: &str) -> Result<Vec<u8>, DomainError> {
        let url = Self::release_url(tag, target);
        http_get_bytes(&url)
    }
}

fn http_get(url: &str) -> Result<String, DomainError> {
    let mut response = ureq::get(url)
        .header("User-Agent", "awesome-schema-cli")
        .header("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| DomainError::DownloadFailed(e.to_string()))?;
    response
        .body_mut()
        .read_to_string()
        .map_err(|e| DomainError::DownloadFailed(e.to_string()))
}

fn http_get_bytes(url: &str) -> Result<Vec<u8>, DomainError> {
    let mut response = ureq::get(url)
        .header("User-Agent", "awesome-schema-cli")
        .call()
        .map_err(|e| DomainError::DownloadFailed(e.to_string()))?;
    response
        .body_mut()
        .read_to_vec()
        .map_err(|e| DomainError::DownloadFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::GithubReleaseClient;

    #[test]
    fn asset_name_matches_release_contract() {
        assert_eq!(
            GithubReleaseClient::asset_name("aarch64-apple-darwin"),
            "awesome-schema-aarch64-apple-darwin"
        );
        assert_eq!(
            GithubReleaseClient::asset_name("x86_64-pc-windows-msvc"),
            "awesome-schema-x86_64-pc-windows-msvc.exe"
        );
    }

    #[test]
    fn release_url_prefixes_v() {
        assert_eq!(
            GithubReleaseClient::release_url("0.1.0-alpha.1", "x86_64-unknown-linux-gnu"),
            "https://github.com/davp00/awsome-schema/releases/download/v0.1.0-alpha.1/awesome-schema-x86_64-unknown-linux-gnu"
        );
    }
}
