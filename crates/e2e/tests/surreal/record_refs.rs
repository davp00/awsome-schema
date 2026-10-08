//! Record reference (`@link` / REFERENCE / COMPUTED) e2e (soft-skips live tests without Docker).

use std::fs;

use predicates::prelude::*;

use super::common::{
    TestProject, field_define, record_ref_schema, require_surrealdb, table_exists,
};

#[tokio::test]
async fn cli_db_push_and_pull_preserves_record_references() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_push_and_pull_preserves_record_references").await
    else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&record_ref_schema(Some(&endpoint)));

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
        pulled.contains("@link(\"PostAuthor\")"),
        "pulled schema should restore named @link pair:\n{pulled}"
    );
    assert!(
        pulled.contains("@onDelete(Cascade)"),
        "pulled schema should restore @onDelete(Cascade):\n{pulled}"
    );

    project.awesome_schema_cmd().arg("validate").assert().success();
}
