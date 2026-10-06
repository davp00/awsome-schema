//! Migration lifecycle e2e against live SurrealDB (soft-skips without Docker).

mod common;

use predicates::prelude::*;

use common::{TestProject, require_surrealdb, schema_with_endpoint, table_exists};

#[tokio::test]
async fn cli_migrate_apply_status_and_rollback() {
    let Some((_container, endpoint)) = require_surrealdb("cli_migrate_apply_status_and_rollback").await
    else {
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
    assert!(
        table_exists(&endpoint, "_awesome_migrations").await,
        "apply should bootstrap the migration ledger table"
    );

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
