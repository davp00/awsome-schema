use clap::Parser;
use cli::app::Cli;
use cli::{commands, output::Printer};

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

fn temp_cli(args: &[&str]) -> (tempfile::TempDir, Cli) {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema = temp.path().join("awesome.schema");
    std::fs::write(&schema, MINIMAL_SCHEMA).expect("write schema");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(&migrations).expect("migrations");

    let schema_path = schema.to_string_lossy().into_owned();
    let migrations_path = migrations.to_string_lossy().into_owned();

    let mut command_argv = vec![
        "awesome-schema",
        "--schema",
        schema_path.as_str(),
        "--migrations-dir",
        migrations_path.as_str(),
    ];
    command_argv.extend(args);

    let cli = Cli::try_parse_from(command_argv).expect("parse cli");
    (temp, cli)
}

#[test]
fn dispatch_init_command() {
    let (temp, cli) = temp_cli(&["init"]);
    commands::dispatch(&cli, &Printer::new()).expect("init");
    assert!(temp.path().join("awesome.schema").exists());
}

#[test]
fn dispatch_init_when_schema_already_exists() {
    let (temp, cli) = temp_cli(&["init"]);
    commands::dispatch(&cli, &Printer::new()).expect("init first");
    commands::dispatch(&cli, &Printer::new()).expect("init second");
    assert!(temp.path().join("awesome.schema").exists());
}

#[test]
fn dispatch_validate_command() {
    let (_temp, cli) = temp_cli(&["validate"]);
    commands::dispatch(&cli, &Printer::new()).expect("validate");
}

#[test]
fn dispatch_format_write_command() {
    let (_temp, cli) = temp_cli(&["format", "--write"]);
    commands::dispatch(&cli, &Printer::new()).expect("format");
}

#[test]
fn dispatch_generate_targets() {
    for target in ["schema", "rust", "typescript"] {
        let (_temp, cli) = temp_cli(&["generate", "--target", target]);
        commands::dispatch(&cli, &Printer::new()).expect("generate");
    }
}

#[test]
fn dispatch_migrate_create_and_status() {
    let (temp, cli) = temp_cli(&["migrate", "create", "manual"]);
    commands::dispatch(&cli, &Printer::new()).expect("create");

    let cli = Cli::try_parse_from([
        "awesome-schema",
        "--schema",
        temp.path().join("awesome.schema").to_str().expect("schema"),
        "--migrations-dir",
        temp.path().join("migrations").to_str().expect("migrations"),
        "migrate",
        "status",
    ])
    .expect("parse status");
    commands::dispatch(&cli, &Printer::new()).expect("status");
}

#[test]
fn dispatch_migrate_dev_creates_migration() {
    let (_temp, cli) = temp_cli(&["migrate", "dev", "--name", "bootstrap"]);
    commands::dispatch(&cli, &Printer::new()).expect("migrate dev");
}

#[test]
fn dispatch_db_pull_returns_not_implemented() {
    let (_temp, cli) = temp_cli(&["db", "pull"]);
    let error = commands::dispatch(&cli, &Printer::new()).expect_err("pull");
    assert!(error.to_string().contains("not implemented"));
}

#[test]
fn dispatch_db_push_returns_connection_error_without_server() {
    let (_temp, cli) = temp_cli(&["db", "push"]);
    let error = commands::dispatch(&cli, &Printer::new()).expect_err("push");
    assert!(error.to_string().contains("database") || error.to_string().contains("connect"));
}

#[test]
fn introspector_returns_not_implemented() {
    use cli::adapters::SurrealDbIntrospector;
    use schema_core::ports::SchemaIntrospector;

    let error = SurrealDbIntrospector.introspect().expect_err("not implemented");
    assert!(matches!(error, schema_core::DomainError::NotImplemented(_)));
}

#[test]
fn filesystem_adapter_roundtrip() {
    use cli::adapters::FsAdapter;
    use schema_core::ports::FileSystemPort;

    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("file.txt");
    let adapter = FsAdapter;
    adapter.create_dir_all(temp.path().to_str().expect("dir")).expect("mkdir");
    adapter.write_string(path.to_str().expect("path"), "hello").expect("write");
    assert!(adapter.exists(path.to_str().expect("path")));
    assert_eq!(adapter.read_to_string(path.to_str().expect("path")).expect("read"), "hello");
}

#[test]
fn schema_source_reads_existing_schema() {
    use cli::adapters::{FsAdapter, SchemaFileSource};
    use schema_core::ports::SchemaSource;
    use std::sync::Arc;

    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("awesome.schema");
    std::fs::write(&path, MINIMAL_SCHEMA).expect("write");

    let source = SchemaFileSource::new(Arc::new(FsAdapter), path.to_string_lossy().into_owned());
    let schema = source.load_schema().expect("schema");
    assert_eq!(schema.models.len(), 1);
    assert_eq!(source.load_raw().expect("raw").len(), MINIMAL_SCHEMA.len());
}

#[test]
fn schema_source_errors_when_schema_missing() {
    use cli::adapters::{FsAdapter, SchemaFileSource};
    use schema_core::ports::SchemaSource;
    use std::sync::Arc;

    let source = SchemaFileSource::new(Arc::new(FsAdapter), "missing.schema".to_owned());
    let error = source.load_schema().expect_err("missing");
    assert!(matches!(error, schema_core::DomainError::SchemaNotFound(_)));
}

#[test]
fn migration_store_returns_empty_when_dir_missing() {
    use cli::adapters::{FsAdapter, MigrationStoreAdapter};
    use schema_core::ports::MigrationStore;
    use std::sync::Arc;

    let store = MigrationStoreAdapter::new(Arc::new(FsAdapter), "missing-migrations".to_owned());
    assert!(store.list_migrations().expect("list").is_empty());
    assert!(store.load_last_snapshot().expect("snapshot").is_none());
}

#[test]
fn migration_store_returns_none_when_snapshot_file_missing() {
    use cli::adapters::{FsAdapter, MigrationStoreAdapter};
    use schema_core::ports::MigrationStore;
    use std::sync::Arc;

    let temp = tempfile::tempdir().expect("tempdir");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(migrations.join("001_init")).expect("mkdir");

    let store =
        MigrationStoreAdapter::new(Arc::new(FsAdapter), migrations.to_string_lossy().into_owned());
    assert!(store.load_last_snapshot().expect("snapshot").is_none());
}

#[test]
fn filesystem_adapter_reports_read_errors() {
    use cli::adapters::FsAdapter;
    use schema_core::ports::FileSystemPort;

    let adapter = FsAdapter;
    let error = adapter.read_to_string("/definitely/missing/path.schema").expect_err("read");
    assert!(matches!(error, schema_core::DomainError::SchemaReadFailed(_)));
}

#[test]
fn migration_store_loads_snapshot_json() {
    use cli::adapters::{FsAdapter, MigrationStoreAdapter, SchemaFileSource};
    use schema_core::ports::{MigrationStore, SchemaSource};
    use std::sync::Arc;

    let temp = tempfile::tempdir().expect("tempdir");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(migrations.join("001_init")).expect("mkdir");

    let fs = Arc::new(FsAdapter);
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");
    let source = SchemaFileSource::new(fs.clone(), schema_path.to_string_lossy().into_owned());
    let schema = source.load_schema().expect("schema");

    let store = MigrationStoreAdapter::new(fs, migrations.to_string_lossy().into_owned());
    store.save_snapshot(&schema, &migrations.join("001_init").to_string_lossy()).expect("save");

    let loaded = store.load_last_snapshot().expect("load").expect("some");
    assert_eq!(loaded.models.len(), 1);
}

#[test]
fn dispatch_migrate_dev_reports_no_changes() {
    let (temp, cli) = temp_cli(&["migrate", "dev"]);
    commands::dispatch(&cli, &Printer::new()).expect("first dev");
    commands::dispatch(&cli, &Printer::new()).expect("second dev");
    assert!(temp.path().join("migrations").exists());
}

#[test]
fn migration_store_lists_and_snapshots() {
    use cli::adapters::{FsAdapter, MigrationStoreAdapter, SchemaFileSource};
    use schema_core::ports::{MigrationStore, SchemaSource};
    use std::sync::Arc;

    let temp = tempfile::tempdir().expect("tempdir");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(&migrations).expect("mkdir");
    std::fs::create_dir_all(migrations.join("001_test")).expect("migration dir");

    let fs = Arc::new(FsAdapter);
    let store = MigrationStoreAdapter::new(fs.clone(), migrations.to_string_lossy().into_owned());
    assert_eq!(store.list_migrations().expect("list"), vec!["001_test".to_owned()]);

    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");
    let source = SchemaFileSource::new(fs, schema_path.to_string_lossy().into_owned());
    let schema = source.load_schema().expect("schema");
    store.save_snapshot(&schema, &migrations.join("001_test").to_string_lossy()).expect("snapshot");
    assert!(store.load_last_snapshot().expect("snapshot").is_some());
}
