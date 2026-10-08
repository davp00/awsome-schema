//! Shared helpers for SurrealDB end-to-end CLI tests.

use std::fs;

use assert_cmd::Command;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use testcontainers_modules::surrealdb::{SURREALDB_PORT, SurrealDb};
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

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
        let mut command = Command::cargo_bin("awesome-schema").expect("binary");
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
    let message = error.to_string();
    message.contains("SocketNotFoundError") || message.contains("docker.sock")
}

pub type SurrealContainer = testcontainers_modules::testcontainers::ContainerAsync<SurrealDb>;

pub async fn start_surrealdb() -> Option<(SurrealContainer, String)> {
    let container = match SurrealDb::default()
        .with_user("root")
        .with_password("root")
        .with_tag("v3.3.0")
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
