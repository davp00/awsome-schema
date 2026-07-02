use assert_cmd::Command;
use predicates::prelude::*;

const MINIMAL_SCHEMA: &str = r#"datasource db {
  provider = "surrealdb"
  url      = "ws://127.0.0.1:8000"
  namespace = "test"
  database  = "main"
}

model User {
  id    @id
  email string @unique
}
"#;

#[test]
fn validate_command_succeeds_for_valid_schema() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(temp.path().join("awesome.schema"), MINIMAL_SCHEMA).expect("write");

    Command::cargo_bin("awesome-schema")
        .expect("binary")
        .current_dir(temp.path())
        .args(["validate", "--schema", "awesome.schema"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Schema is valid"));
}

#[test]
fn format_command_prints_schema_without_writing_by_default() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(temp.path().join("awesome.schema"), "model User {\n  id @id\n}\n\n")
        .expect("write");

    Command::cargo_bin("awesome-schema")
        .expect("binary")
        .current_dir(temp.path())
        .args(["format", "--schema", "awesome.schema"])
        .assert()
        .success()
        .stdout(predicate::str::contains("model User"));
}

#[test]
fn migrate_status_reports_empty_state() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(temp.path().join("awesome.schema"), MINIMAL_SCHEMA).expect("write");
    std::fs::create_dir_all(temp.path().join("migrations")).expect("migrations dir");

    Command::cargo_bin("awesome-schema")
        .expect("binary")
        .current_dir(temp.path())
        .args(["migrate", "status", "--schema", "awesome.schema", "--migrations-dir", "migrations"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No migrations found"));
}

#[test]
fn printer_methods_are_exercised_by_commands() {
    use cli::output::Printer;

    let printer = Printer::new();
    printer.success("ok");
    printer.info("info");
    printer.error("err");
    printer.plain("plain");
}
