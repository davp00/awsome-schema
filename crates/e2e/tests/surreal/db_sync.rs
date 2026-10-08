//! `db push` / `db pull` e2e against live SurrealDB (soft-skips without Docker).

use std::fs;
use std::path::Path;

use predicates::prelude::*;

use super::common::{TestProject, require_surrealdb, schema_with_endpoint, table_exists};

#[tokio::test]
async fn cli_db_push_applies_schema_to_surrealdb() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_push_applies_schema_to_surrealdb").await
    else {
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
async fn cli_db_pull_overwrites_schema_after_push() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_pull_overwrites_schema_after_push").await
    else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&schema_with_endpoint(&endpoint));

    project.awesome_schema_cmd().arg("db").arg("push").assert().success();

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled"));

    project.awesome_schema_cmd().arg("validate").assert().success();
}

#[tokio::test]
async fn cli_db_pull_split_by_table_writes_schema_directory() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_pull_split_by_table_writes_schema_directory").await
    else {
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

    assert!(Path::new(&project.path("schema/_config.schema")).exists());
    assert!(Path::new(&project.path("schema/tables/user.schema")).exists());

    let config = fs::read_to_string(project.path("schema/_config.schema")).expect("config");
    assert!(config.contains("datasource db"));
    assert!(config.contains(&format!("\"{endpoint}\"")));

    let user = fs::read_to_string(project.path("schema/tables/user.schema")).expect("user");
    assert!(user.contains("model User"));
    assert!(user.contains("@id"));

    project
        .awesome_schema_cmd_with_schema("schema")
        .arg("validate")
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema is valid"));
}
