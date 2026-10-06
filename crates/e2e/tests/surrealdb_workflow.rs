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
        self.awesome_schema_cmd_with_schema("awesome.schema")
    }

    fn awesome_schema_cmd_with_schema(&self, schema: &str) -> Command {
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

async fn start_surrealdb()
-> Option<(testcontainers_modules::testcontainers::ContainerAsync<SurrealDb>, String)> {
    let container = match SurrealDb::default()
        .with_user("root")
        .with_password("root")
        .with_tag("v3.3.0")
        .start()
        .await
    {
        Ok(container) => container,
        Err(error) if docker_unavailable(&error) => {
            eprintln!("skipping SurrealDB e2e: Docker unavailable ({error})");
            return None;
        }
        Err(error) => panic!("start surrealdb: {error}"),
    };

    let port = container.get_host_port_ipv4(SURREALDB_PORT).await.expect("port");

    let url = format!("127.0.0.1:{port}");
    Some((container, url))
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
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

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

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("apply")
        .assert()
        .success()
        .stdout(predicate::str::contains("Applied 0 migrations (1 already applied)"));

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("[applied]"))
        .stdout(predicate::str::contains("1 applied, 0 pending"));

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("rollback")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rolled back 1 migration"));

    assert!(!table_exists(&endpoint, "user").await);

    project
        .awesome_schema_cmd()
        .arg("migrate")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("[pending]"))
        .stdout(predicate::str::contains("0 applied, 1 pending"));
}

#[tokio::test]
async fn cli_db_push_applies_schema_to_surrealdb() {
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

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
async fn cli_db_pull_overwrites_schema_after_push() {
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&schema_with_endpoint(&endpoint));

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("push")
        .assert()
        .success();

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled"));

    project
        .awesome_schema_cmd()
        .arg("validate")
        .assert()
        .success();
}

#[tokio::test]
async fn cli_db_pull_split_by_table_writes_schema_directory() {
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&schema_with_endpoint(&endpoint));

    project.awesome_schema_cmd().arg("db").arg("push").assert().success();

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--split-by-table")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled").and(predicate::str::contains("schema")));

    assert!(Path::new(&project.path("schema/_config.awesome.schema")).exists());
    assert!(Path::new(&project.path("schema/tables/user.awesome.schema")).exists());

    let config = fs::read_to_string(project.path("schema/_config.awesome.schema")).expect("config");
    assert!(config.contains("datasource db"));
    assert!(config.contains(&format!("\"{endpoint}\"")));

    let user = fs::read_to_string(project.path("schema/tables/user.awesome.schema")).expect("user");
    assert!(user.contains("model User"));
    assert!(user.contains("@id"));

    project
        .awesome_schema_cmd_with_schema("schema")
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema is valid"));
}

#[tokio::test]
async fn cli_db_push_and_pull_preserves_relation_edge() {
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

    let schema = format!(
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
    );

    let project = TestProject::new();
    project.write_schema(&schema);

    project.awesome_schema_cmd().arg("db").arg("push").assert().success();
    assert!(table_exists(&endpoint, "likes").await);

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled"));

    let pulled = fs::read_to_string(project.path("awesome.schema")).expect("pulled schema");
    assert!(pulled.contains("edge Likes"));
    assert!(pulled.contains("in  User") || pulled.contains("in User"));
    assert!(pulled.contains("out Post") || pulled.contains("out Post"));

    project.awesome_schema_cmd().arg("validate").assert().success();
}

#[tokio::test]
async fn cli_generate_emits_reference_and_computed_link() {
    let schema = r#"datasource db {
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

    let project = TestProject::new();
    project.write_schema(schema);

    project
        .awesome_schema_cmd()
        .arg("generate")
        .arg("--target")
        .arg("schema")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "DEFINE FIELD author ON post TYPE record<user> REFERENCE ON DELETE CASCADE;",
        ))
        .stdout(predicate::str::contains(
            "DEFINE FIELD posts ON user COMPUTED <~(post FIELD author);",
        ));
}

#[tokio::test]
async fn cli_db_push_and_pull_preserves_record_references() {
    let Some((_container, endpoint)) = start_surrealdb().await else {
        return;
    };

    let schema = format!(
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
  id    @id
  posts Post[] @link("PostAuthor")
  @@table(schemafull)
}}

model Post {{
  id     @id
  author User @link("PostAuthor") @onDelete(Cascade)
  @@table(schemafull)
}}
"#
    );

    let project = TestProject::new();
    project.write_schema(&schema);

    project
        .awesome_schema_cmd()
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema is valid"));

    project.awesome_schema_cmd().arg("db").arg("push").assert().success();
    assert!(table_exists(&endpoint, "user").await);
    assert!(table_exists(&endpoint, "post").await);

    let author_define = field_define(&endpoint, "post", "author").await;
    assert!(
        author_define.to_ascii_lowercase().contains("reference"),
        "expected REFERENCE on author, got: {author_define}"
    );
    assert!(
        author_define.to_ascii_lowercase().contains("cascade"),
        "expected ON DELETE CASCADE on author, got: {author_define}"
    );

    let posts_define = field_define(&endpoint, "user", "posts").await;
    assert!(
        posts_define.to_ascii_lowercase().contains("computed"),
        "expected COMPUTED on posts, got: {posts_define}"
    );
    assert!(
        posts_define.contains("<~") && posts_define.to_ascii_lowercase().contains("author"),
        "expected backlink to author, got: {posts_define}"
    );

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled"));

    let pulled = fs::read_to_string(project.path("awesome.schema")).expect("pulled schema");
    assert!(
        pulled.contains("@link"),
        "pulled schema should restore @link fields:\n{pulled}"
    );
    assert!(
        pulled.contains("@onDelete(Cascade)") || pulled.contains("onDelete"),
        "pulled schema should restore onDelete Cascade:\n{pulled}"
    );

    project.awesome_schema_cmd().arg("validate").assert().success();
}

async fn field_define(endpoint: &str, table: &str, field: &str) -> String {
    let db = Surreal::new::<Ws>(endpoint.to_owned()).await.expect("connect");
    db.signin(Root { username: "root".to_owned(), password: "root".to_owned() })
        .await
        .expect("signin");
    db.use_ns("test").use_db("main").await.expect("use ns/db");

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

#[test]
fn parser_rejects_record_id_syntax_in_e2e_crate() {
    let error = parser::parse(r"model User { id RecordId<User> @id }").expect_err("reject");
    assert!(matches!(error, DomainError::ParseError(_)));
}
