//! Offline CLI e2e (no Docker / SurrealDB required).

mod common;

use std::path::Path;

use predicates::prelude::*;

use common::{TestProject, minimal_schema, record_ref_schema};

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
async fn cli_generate_outputs_schema_sql() {
    let project = TestProject::new();
    project.write_schema(minimal_schema());

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
async fn cli_generate_emits_reference_and_computed_link() {
    let project = TestProject::new();
    project.write_schema(&record_ref_schema(None));

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
