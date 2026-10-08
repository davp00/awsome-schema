//! Graph edge (`TYPE RELATION`) e2e against live SurrealDB (soft-skips without Docker).

use std::fs;

use predicates::prelude::*;

use super::common::{TestProject, edge_schema, require_surrealdb, table_exists};

#[tokio::test]
async fn cli_db_push_and_pull_preserves_relation_edge() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_push_and_pull_preserves_relation_edge").await
    else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&edge_schema(&endpoint));

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
    assert!(pulled.contains("edge Likes"), "missing edge Likes:\n{pulled}");
    assert!(
        pulled.contains("in  User") || pulled.contains("in User"),
        "missing in User:\n{pulled}"
    );
    assert!(
        pulled.contains("out Post") || pulled.contains("out Post"),
        "missing out Post:\n{pulled}"
    );
    assert!(
        pulled.contains("score") && pulled.contains("int"),
        "edge field score int should survive pull:\n{pulled}"
    );

    project.awesome_schema_cmd().arg("validate").assert().success();
}
