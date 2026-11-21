//! Shared helpers for SurrealDB end-to-end CLI tests.

use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;

use assert_cmd::Command;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use testcontainers_modules::surrealdb::{SURREALDB_PORT, SurrealDb};
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

/// Locate the workspace `awesome-schema` binary.
///
/// `assert_cmd::Command::cargo_bin` needs `CARGO_BIN_EXE_*`, which Cargo only
/// sets for bins of the package under test — not for a dependency CLI. Its
/// legacy `target/deps` walk also breaks under Cargo's newer `build-dir`
/// layout (`target/debug/build/<pkg>/<hash>/out/…`).
fn find_awesome_schema_bin() -> PathBuf {
    let file_name = format!("awesome-schema{}", std::env::consts::EXE_SUFFIX);

    for key in ["CARGO_BIN_EXE_awesome-schema", "CARGO_BIN_EXE_awesome_schema"] {
        if let Some(path) = std::env::var_os(key) {
            let path = PathBuf::from(path);
            if path.is_file() {
                return path;
            }
        }
    }

    let mut candidates = Vec::new();
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        for profile in ["debug", "release"] {
            candidates.push(PathBuf::from(&target_dir).join(profile).join(&file_name));
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for profile in ["debug", "release"] {
        candidates.push(manifest_dir.join("../../target").join(profile).join(&file_name));
    }

    // Walk up from the test executable (covers target/debug/deps and
    // target/debug/build/<pkg>/<hash>/out layouts).
    if let Ok(mut dir) = std::env::current_exe().map(|p| p) {
        for _ in 0..10 {
            if !dir.pop() {
                break;
            }
            for profile in ["debug", "release"] {
                candidates.push(dir.join(profile).join(&file_name));
                candidates.push(dir.join("target").join(profile).join(&file_name));
            }
            candidates.push(dir.join(&file_name));
        }
    }

    for candidate in &candidates {
        if let Ok(canonical) = candidate.canonicalize() {
            if canonical.is_file() {
                return canonical;
            }
        } else if candidate.is_file() {
            return candidate.clone();
        }
    }

    panic!(
        "could not find `{file_name}` for e2e tests; tried {} candidates (build with `cargo build -p awesome-schema`)",
        candidates.len()
    );
}

fn awesome_schema_exe() -> PathBuf {
    find_awesome_schema_bin()
}

pub const MINIMAL_SCHEMA: &str = r#"datasource db {
  provider = "surrealdb"
  url      = env("SURREALDB_URL")
  namespace = "test"
  database  = "main"
}

naming {
  tables = "snake_case"
}

model User {
  id    @id
  email string @unique

  @@table(schemafull)
}
"#;

pub const RECORD_REF_SCHEMA_TEMPLATE: &str = r#"datasource db {
  provider = "surrealdb"
  url      = env("SURREALDB_URL")
  namespace = "test"
  database  = "main"
}

naming {
  tables = "snake_case"
}

model User {
  id    @id
  posts Post[] @link("PostAuthor")
  @@table(schemafull)
}

model Post {
  id     @id
  author User @link("PostAuthor") @onDelete(Cascade)
  @@table(schemafull)
}
"#;

pub fn minimal_schema() -> &'static str {
    MINIMAL_SCHEMA
}

pub fn schema_with_endpoint(endpoint: &str) -> String {
    MINIMAL_SCHEMA.replace("env(\"SURREALDB_URL\")", &format!("\"{endpoint}\""))
}

pub fn record_ref_schema(endpoint: Option<&str>) -> String {
    match endpoint {
        Some(url) => RECORD_REF_SCHEMA_TEMPLATE
            .replace("env(\"SURREALDB_URL\")", &format!("\"{url}\"")),
        None => RECORD_REF_SCHEMA_TEMPLATE.to_owned(),
    }
}

pub fn edge_schema(endpoint: &str) -> String {
    format!(
        r#"datasource db {{
  provider = "surrealdb"
  url      = "{endpoint}"
  namespace = "test"
  database  = "main"
}}

naming {{
  tables = "snake_case"
}}

model User {{
  id @id
  liked Post[] @relation("Likes")
  @@table(schemafull)
}}

model Post {{
  id @id
  @@table(schemafull)
}}

edge Likes {{
  in  User
  out Post
  score int
  @@table(schemafull)
}}
"#
    )
}

pub struct TestProject {
    root: tempfile::TempDir,
}

impl TestProject {
    pub fn new() -> Self {
        Self { root: tempfile::tempdir().expect("tempdir") }
    }

    pub fn path(&self, relative: &str) -> String {
        self.root.path().join(relative).to_string_lossy().into_owned()
    }

    pub fn write_schema(&self, content: &str) {
        fs::write(self.path("awesome.schema"), content).expect("write schema");
    }

    pub fn awesome_schema_cmd(&self) -> Command {
        self.awesome_schema_cmd_with_schema("awesome.schema")
    }

    pub fn awesome_schema_cmd_with_schema(&self, schema: &str) -> Command {
        let mut command = Command::from_std(StdCommand::new(awesome_schema_exe()));
        command
            .current_dir(self.root.path())
            .arg("--schema")
            .arg(schema)
            .arg("--migrations-dir")
            .arg("migrations");
        command
    }
}

fn docker_unavailable(error: &impl std::fmt::Display) -> bool {
    let message = error.to_string().to_lowercase();
    message.contains("socketnotfounderror")
        || message.contains("docker.sock")
        || message.contains("client error (connect)")
        || message.contains("connection refused")
}

#[cfg(test)]
mod tests {
    use super::docker_unavailable;

    #[test]
    fn treats_unreachable_docker_as_unavailable() {
        assert!(docker_unavailable(&"SocketNotFoundError: /var/run/docker.sock"));
        assert!(docker_unavailable(
            &"Error in the hyper legacy client: client error (Connect)"
        ));
        assert!(!docker_unavailable(&"image pull denied"));
    }

    #[test]
    fn resolve_surreal_tag_prefers_image_then_tag() {
        assert_eq!(
            super::resolve_surreal_tag(Some("surrealdb/surrealdb:v3.3.0"), Some("ignored")),
            "v3.3.0"
        );
        assert_eq!(
            super::resolve_surreal_tag(Some("surrealdb/surrealdb:v4"), None),
            "v4"
        );
        assert_eq!(super::resolve_surreal_tag(None, Some("v3.3.0")), "v3.3.0");
        assert_eq!(super::resolve_surreal_tag(None, None), "v4");
        assert_eq!(super::resolve_surreal_tag(Some("  "), Some("v3.3.0")), "v3.3.0");
    }

    #[test]
    fn finds_awesome_schema_binary() {
        let path = super::find_awesome_schema_bin();
        assert!(
            path.is_file(),
            "expected built CLI at {}",
            path.display()
        );
    }
}

pub type SurrealContainer = testcontainers_modules::testcontainers::ContainerAsync<SurrealDb>;

/// Resolve a Docker tag from optional `SURREAL_IMAGE` / `SURREAL_TAG` values.
///
/// Prefer a full image (`surrealdb/surrealdb:v4` or `…:v3.3.0`); otherwise a bare
/// tag; default `v4` (4.x line).
pub fn resolve_surreal_tag(image: Option<&str>, tag: Option<&str>) -> String {
    if let Some(image) = image.map(str::trim).filter(|value| !value.is_empty()) {
        return image
            .rsplit_once(':')
            .map(|(_, tag)| tag.to_owned())
            .unwrap_or_else(|| image.to_owned());
    }
    tag.map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| "v4".to_owned())
}

/// Image/tag for Surreal e2e containers from process env.
pub fn surreal_image_tag() -> String {
    resolve_surreal_tag(
        std::env::var("SURREAL_IMAGE").ok().as_deref(),
        std::env::var("SURREAL_TAG").ok().as_deref(),
    )
}

pub async fn start_surrealdb() -> Option<(SurrealContainer, String)> {
    let tag = surreal_image_tag();
    let container = match SurrealDb::default()
        .with_user("root")
        .with_password("root")
        .with_tag(tag)
        .start()
        .await
    {
        Ok(container) => container,
        Err(error) if docker_unavailable(&error) => {
            return None;
        }
        Err(error) => panic!("start surrealdb: {error}"),
    };

    let port = container.get_host_port_ipv4(SURREALDB_PORT).await.expect("port");
    let url = format!("127.0.0.1:{port}");
    Some((container, url))
}

/// Soft-skip helper: print a consistent message and return `None` when Docker is unavailable.
pub async fn require_surrealdb(test_name: &str) -> Option<(SurrealContainer, String)> {
    match start_surrealdb().await {
        Some(pair) => Some(pair),
        None => {
            eprintln!("skipping `{test_name}`: Docker unavailable");
            None
        }
    }
}

async fn connect(endpoint: &str) -> Surreal<Client> {
    let db = Surreal::new::<Ws>(endpoint.to_owned()).await.expect("connect");
    db.signin(Root { username: "root".to_owned(), password: "root".to_owned() })
        .await
        .expect("signin");
    db.use_ns("test").use_db("main").await.expect("use ns/db");
    db
}

pub async fn table_exists(endpoint: &str, table: &str) -> bool {
    let db = connect(endpoint).await;
    let mut response = db.query("INFO FOR DB;").await.expect("info for db");
    let info: Option<serde_json::Value> = response.take(0).expect("take");
    let Some(info) = info else {
        return false;
    };
    info.get("tables").and_then(|tables| tables.get(table)).is_some()
}

pub async fn field_define(endpoint: &str, table: &str, field: &str) -> String {
    let db = connect(endpoint).await;
    let mut response = db
        .query(format!("INFO FOR TABLE {table};"))
        .await
        .expect("info for table");
    let info: Option<serde_json::Value> = response.take(0).expect("take");
    let info = info.expect("table info");
    info.get("fields")
        .and_then(|fields| fields.get(field))
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_owned()
}
