//! End-to-end workflow tests against a real `SurrealDB` instance via testcontainers.

use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use schema_core::DomainError;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use testcontainers_modules::surrealdb::{SURREALDB_PORT, SurrealDb};
use testcontainers_modules::testcontainers::ImageExt;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

const MINIMAL_SCHEMA: &str = r#"datasource db {
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

fn schema_with_endpoint(endpoint: &str) -> String {
    MINIMAL_SCHEMA.replace("env(\"SURREALDB_URL\")", &format!("\"{endpoint}\""))
}

struct TestProject {
    root: tempfile::TempDir,
}

impl TestProject {
    fn new() -> Self {
        Self { root: tempfile::tempdir().expect("tempdir") }
    }

    fn path(&self, relative: &str) -> String {
        self.root.path().join(relative).to_string_lossy().into_owned()
    }

    fn write_schema(&self, content: &str) {
        fs::write(self.path("awesome.schema"), content).expect("write schema");
    }

    fn awesome_schema_cmd(&self) -> Command {
        let mut command = Command::cargo_bin("awesome-schema").expect("binary");
        command
            .current_dir(self.root.path())
            .arg("--schema")
            .arg("awesome.schema")
            .arg("--migrations-dir")
            .arg("migrations");
        command
    }
}

async fn start_surrealdb()
-> (testcontainers_modules::testcontainers::ContainerAsync<SurrealDb>, String) {
    let container = SurrealDb::default()
        .with_user("root")
        .with_password("root")
        .with_tag("v3.3.0")
        .start()
        .await
        .expect("start surrealdb");

    let port = container.get_host_port_ipv4(SURREALDB_PORT).await.expect("port");

    let url = format!("127.0.0.1:{port}");
    (container, url)
}

async fn table_exists(endpoint: &str, table: &str) -> bool {
    let db = Surreal::new::<Ws>(endpoint.to_owned()).await.expect("connect");
    db.signin(Root { username: "root".to_owned(), password: "root".to_owned() })
        .await
        .expect("signin");
    db.use_ns("test").use_db("main").await.expect("use ns/db");

    let mut response = db.query("INFO FOR DB;").await.expect("info for db");
    let info: Option<serde_json::Value> = response.take(0).expect("take");
    let Some(info) = info else {
        return false;
    };

    info.get("tables").and_then(|tables| tables.get(table)).is_some()
}

#[tokio::test]
async fn cli_init_validate_and_migrate_against_surrealdb() {
    let (_container, endpoint) = start_surrealdb().await;

    let project = TestProject::new();
    project.write_schema(&schema_with_endpoint(&endpoint));

    project
        .awesome_schema_cmd()
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema is valid"));

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("dev")
        .arg("--name")
        .arg("create_user")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created migration"));

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("apply")
        .assert()
        .success()
        .stdout(predicate::str::contains("Applied 1 migrations"));

    assert!(table_exists(&endpoint, "user").await);
}

#[tokio::test]
async fn cli_db_push_applies_schema_to_surrealdb() {
    let (_container, endpoint) = start_surrealdb().await;

    let project = TestProject::new();
    project.write_schema(&schema_with_endpoint(&endpoint));

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("push")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pushed schema to database"));

    assert!(table_exists(&endpoint, "user").await);
}

#[tokio::test]
async fn cli_generate_outputs_schema_sql() {
    let project = TestProject::new();
    project.write_schema(MINIMAL_SCHEMA);

    project
        .awesome_schema_cmd()
        .arg("generate")
        .arg("--target")
        .arg("schema")
        .assert()
        .success()
        .stdout(predicate::str::contains("DEFINE TABLE user SCHEMAFULL"));
}

#[tokio::test]
async fn cli_init_creates_project_files() {
    let project = TestProject::new();

    project
        .awesome_schema_cmd()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    assert!(Path::new(&project.path("awesome.schema")).exists());
    assert!(Path::new(&project.path("migrations")).is_dir());
}

#[tokio::test]
async fn db_pull_is_not_implemented_yet() {
    let project = TestProject::new();
    project.write_schema(MINIMAL_SCHEMA);

    project.awesome_schema_cmd().arg("db").arg("pull").assert().failure().stderr(
        predicate::str::contains("not implemented").or(predicate::str::contains("NOT_IMPLEMENTED")),
    );
}

#[test]
fn parser_rejects_record_id_syntax_in_e2e_crate() {
    let error = parser::parse(r"model User { id RecordId<User> @id }").expect_err("reject");
    assert!(matches!(error, DomainError::ParseError(_)));
}
