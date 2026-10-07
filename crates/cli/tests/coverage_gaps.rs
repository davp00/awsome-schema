use std::sync::Arc;

use cli::adapters::{list_schema_files, FsAdapter, SchemaFileSource};
use cli::commands::{db, format, migrate};
use cli::output::Printer;
use cli::schema_layout::{split_schema_dir, table_fragment_path};
use cli::test_context_with_introspector;
use schema_core::ports::{FileSystemPort, MigrationLedger, SchemaIntrospector, SchemaSource};
use schema_core::{
    AppliedMigration, DatabaseConfig, DatabaseSchema, DomainError, Edge, Field, FieldType,
    InitProjectUseCase, MigrateApplyUseCase, MigrateRollbackUseCase, MigrateStatusUseCase, Model,
    TableMode,
};

const MINIMAL_SCHEMA: &str = r#"datasource db {
  provider = "surrealdb"
  url      = "ws://127.0.0.1:8000"
  namespace = "test"
  database  = "main"
}

model User {
  id @id
}
"#;

struct StubIntrospector {
    schema: DatabaseSchema,
}

impl SchemaIntrospector for StubIntrospector {
    fn introspect(
        &self,
        _config: &schema_core::DatabaseConfig,
        _preserve: &DatabaseSchema,
    ) -> Result<DatabaseSchema, DomainError> {
        Ok(self.schema.clone())
    }
}

struct EmptyLedger;

impl MigrationLedger for EmptyLedger {
    fn ensure_schema(&self, _config: &DatabaseConfig) -> Result<(), DomainError> {
        Ok(())
    }

    fn list_applied(&self, _config: &DatabaseConfig) -> Result<Vec<AppliedMigration>, DomainError> {
        Ok(Vec::new())
    }

    fn record_applied(
        &self,
        _config: &DatabaseConfig,
        _name: &str,
        _checksum: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    fn remove_applied(&self, _config: &DatabaseConfig, _name: &str) -> Result<(), DomainError> {
        Ok(())
    }
}

fn sample_pulled_schema() -> DatabaseSchema {
    let mut schema = DatabaseSchema::empty();
    schema.datasource.provider = "surrealdb".into();
    schema.datasource.url = Some("ws://127.0.0.1:8000".into());
    schema.datasource.namespace = Some("test".into());
    schema.datasource.database = Some("main".into());
    schema.models.push(Model {
        name: "User".into(),
        fields: vec![Field {
            name: "id".into(),
            field_type: FieldType::RecordId("User".into()),
            optional: false,
            unique: false,
            is_id: true,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: Default::default(),
        }],
        table_mode: TableMode::Schemafull,
        permissions: None,
        indexes: vec![],
        attributes: Default::default(),
    });
    schema.edges.push(Edge {
        name: "Likes".into(),
        in_model: "User".into(),
        out_model: "User".into(),
        fields: vec![],
        table_mode: TableMode::Schemafull,
        permissions: None,
        attributes: Default::default(),
    });
    schema
}

#[test]
fn split_schema_dir_keeps_directory_path_and_builds_table_fragment() {
    assert_eq!(split_schema_dir("/tmp/project/schema"), "/tmp/project/schema");
    assert_eq!(
        table_fragment_path("/tmp/project/schema", "user"),
        "/tmp/project/schema/tables/user.schema"
    );
    // Bare `file.schema` has an empty parent → fall back to `.`.
    assert_eq!(split_schema_dir("awesome.schema"), "./schema");
}

#[test]
fn fs_adapter_write_remove_and_error_paths() {
    let temp = tempfile::tempdir().expect("tempdir");
    let adapter = FsAdapter;
    let dir = temp.path().join("nested");
    let file = dir.join("file.txt");

    adapter
        .create_dir_all(dir.to_str().expect("dir"))
        .expect("mkdir");
    adapter
        .write_string(file.to_str().expect("file"), "hello")
        .expect("write");
    assert!(adapter.exists(file.to_str().expect("file")));

    adapter.remove_file(file.to_str().expect("file")).expect("remove");
    let remove_err = adapter
        .remove_file(file.to_str().expect("file"))
        .expect_err("missing remove");
    assert!(matches!(remove_err, DomainError::WriteFailed(_)));

    let list_err = adapter.list_dir(temp.path().join("missing").to_str().unwrap());
    assert!(matches!(list_err, Err(DomainError::SchemaReadFailed(_))));

    let write_err = adapter.write_string("/definitely/no/such/dir/file.txt", "x");
    assert!(matches!(write_err, Err(DomainError::WriteFailed(_))));

    // create_dir_all fails when a file occupies the path.
    let blocker = temp.path().join("as_dir");
    adapter
        .write_string(blocker.to_str().unwrap(), "x")
        .expect("blocker");
    let mkdir_err = adapter.create_dir_all(blocker.join("child").to_str().unwrap());
    assert!(matches!(mkdir_err, Err(DomainError::WriteFailed(_))));
}

#[test]
fn schema_file_source_directory_and_skip_non_schema() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_dir = temp.path().join("schema");
    std::fs::create_dir_all(schema_dir.join("tables")).expect("tables");

    let fs = Arc::new(FsAdapter);
    let source = SchemaFileSource::new(fs.clone(), schema_dir.to_string_lossy().into_owned());
    let raw_err = source.load_raw().expect_err("dir raw");
    assert!(matches!(raw_err, DomainError::SchemaReadFailed(_)));

    let missing_config = source.load_schema().expect_err("missing config");
    assert!(matches!(missing_config, DomainError::SchemaNotFound(_)));

    std::fs::write(
        schema_dir.join("_config.schema"),
        r#"datasource db { provider = "surrealdb" url = "127.0.0.1:8000" }
"#,
    )
    .expect("config");
    std::fs::write(
        schema_dir.join("tables/user.schema"),
        "model User { id @id }\n",
    )
    .expect("user");
    std::fs::write(schema_dir.join("tables/notes.txt"), "ignore\n").expect("notes");

    let schema = source.load_schema().expect("load");
    assert_eq!(schema.models.len(), 1);

    let files = list_schema_files(&*fs, schema_dir.to_str().unwrap()).expect("list");
    assert!(files.iter().any(|p| p.ends_with("_config.schema")));
    assert!(files.iter().any(|p| p.ends_with("user.schema")));
    assert!(!files.iter().any(|p| p.ends_with("notes.txt")));
}

#[test]
fn printer_default_and_all_methods() {
    let printer = Printer::default();
    printer.warn("warn");
    printer.error("err");
    printer.info("info");
    printer.plain("plain");
    printer.success("ok");
}

#[test]
fn run_pull_warns_when_lossy() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(
        &schema_path,
        r#"datasource db {
  provider = "surrealdb"
  url = "ws://127.0.0.1:8000"
  namespace = "test"
  database = "main"
}
model User { id @id }
edge Likes {
  in User
  out User
}
"#,
    )
    .expect("write");

    // Preserve has edges; stub returns none → lossy warn path.
    let mut pulled = sample_pulled_schema();
    pulled.edges.clear();

    let context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
        Arc::new(StubIntrospector { schema: pulled }),
    )
    .expect("context");

    db::run_pull(&context, false, true, &Printer::new()).expect("pull");
}

#[test]
fn split_by_table_refuses_without_force_and_writes_with_force() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");

    let schema_dir = temp.path().join("schema");
    let tables_dir = schema_dir.join("tables");
    std::fs::create_dir_all(&tables_dir).expect("tables");
    std::fs::write(tables_dir.join("stale.schema"), "model Stale { id @id }\n").expect("stale");
    std::fs::write(tables_dir.join("keep.txt"), "keep\n").expect("keep");

    let context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");

    let refuse = db::run_pull(&context, true, false, &Printer::new()).expect_err("refuse");
    assert!(refuse.to_string().contains("without --force"));

    db::run_pull(&context, true, true, &Printer::new()).expect("force pull");
    assert!(tables_dir.join("user.schema").exists());
    assert!(tables_dir.join("likes.schema").exists());
    assert!(!tables_dir.join("stale.schema").exists());
    assert!(tables_dir.join("keep.txt").exists());
}

#[test]
fn migrate_status_empty_and_rollback_zero_steps() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(&migrations).expect("migrations");

    let mut context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        migrations.to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");

    let store = Arc::new(cli::adapters::MigrationStoreAdapter::new(
        context.filesystem.clone(),
        migrations.to_string_lossy().into_owned(),
    ));
    context.migrate_status = MigrateStatusUseCase::new(store, Arc::new(EmptyLedger));

    migrate::run_status(&context, &Printer::new()).expect("status");
    migrate::run_rollback(&context, 0, &Printer::new()).expect("rollback 0");
}

#[test]
fn fs_adapter_write_fails_when_path_is_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let adapter = FsAdapter;
    let dir = temp.path().join("as_file");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let err = adapter
        .write_string(dir.to_str().expect("dir"), "x")
        .expect_err("dir write");
    assert!(matches!(err, DomainError::WriteFailed(_)));
}

#[test]
fn migration_store_invalid_snapshot_and_list_errors() {
    use cli::adapters::MigrationStoreAdapter;
    use schema_core::ports::MigrationStore;

    let temp = tempfile::tempdir().expect("tempdir");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(migrations.join("001_bad")).expect("dir");
    std::fs::write(migrations.join("001_bad/snapshot.json"), "{not-json").expect("bad json");

    let store = MigrationStoreAdapter::new(
        Arc::new(FsAdapter),
        migrations.to_string_lossy().into_owned(),
    );
    let err = store.load_last_snapshot().expect_err("bad json");
    assert!(matches!(err, DomainError::MigrationError(_)));

    // list_migrations fails when migrations_dir is a file.
    let file_path = temp.path().join("not_a_dir");
    std::fs::write(&file_path, "x").expect("file");
    let store = MigrationStoreAdapter::new(
        Arc::new(FsAdapter),
        file_path.to_string_lossy().into_owned(),
    );
    let err = store.list_migrations().expect_err("not a dir");
    assert!(matches!(err, DomainError::MigrationError(_)));
}

#[test]
fn schema_file_source_config_only_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_dir = temp.path().join("schema");
    std::fs::create_dir_all(&schema_dir).expect("schema dir");
    std::fs::write(
        schema_dir.join("_config.schema"),
        r#"datasource db { provider = "surrealdb" url = "127.0.0.1:8000" }
"#,
    )
    .expect("config");
    let fs = Arc::new(FsAdapter);
    let source = SchemaFileSource::new(fs, schema_dir.to_string_lossy().into_owned());
    let schema = source.load_schema().expect("config only");
    assert!(schema.models.is_empty());
}

#[test]
fn list_schema_files_directory_without_tables_subdir() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_dir = temp.path().join("schema");
    std::fs::create_dir_all(&schema_dir).expect("schema dir");
    std::fs::write(
        schema_dir.join("_config.schema"),
        r#"datasource db { provider = "surrealdb" url = "127.0.0.1:8000" }
"#,
    )
    .expect("config");
    let fs = Arc::new(FsAdapter);
    let files = list_schema_files(&*fs, schema_dir.to_str().unwrap()).expect("list");
    assert_eq!(files.len(), 1);
    assert!(files[0].ends_with("_config.schema"));
}

#[test]
fn schema_file_source_loads_edges_from_directory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_dir = temp.path().join("schema");
    std::fs::create_dir_all(schema_dir.join("tables")).expect("tables");
    std::fs::write(
        schema_dir.join("_config.schema"),
        r#"datasource db { provider = "surrealdb" url = "127.0.0.1:8000" }
"#,
    )
    .expect("config");
    std::fs::write(
        schema_dir.join("tables/user.schema"),
        "model User { id @id }\n",
    )
    .expect("user");
    std::fs::write(
        schema_dir.join("tables/likes.schema"),
        "edge Likes {\n  in User\n  out User\n}\n",
    )
    .expect("likes");

    let fs = Arc::new(FsAdapter);
    let source = SchemaFileSource::new(fs, schema_dir.to_string_lossy().into_owned());
    let schema = source.load_schema().expect("load");
    assert_eq!(schema.models.len(), 1);
    assert_eq!(schema.edges.len(), 1);
    assert_eq!(schema.edges[0].name, "Likes");
}

#[test]
fn format_schema_directory_success_message() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_dir = temp.path().join("schema");
    std::fs::create_dir_all(schema_dir.join("tables")).expect("tables");
    std::fs::write(
        schema_dir.join("_config.schema"),
        r#"datasource db { provider = "surrealdb" url = "127.0.0.1:8000" }
"#,
    )
    .expect("config");
    std::fs::write(
        schema_dir.join("tables/user.schema"),
        "model User { id @id }\n",
    )
    .expect("user");

    let context = cli::di::AppContext::with_paths(
        schema_dir.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
    )
    .expect("context");

    format::run(&context, true, &Printer::new()).expect("format");
}

struct OkExecutor;

impl schema_core::ports::DatabaseExecutor for OkExecutor {
    fn execute_script(
        &self,
        _config: &DatabaseConfig,
        _script: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

struct StatusLedger {
    rows: Vec<AppliedMigration>,
}

impl MigrationLedger for StatusLedger {
    fn ensure_schema(&self, _config: &DatabaseConfig) -> Result<(), DomainError> {
        Ok(())
    }

    fn list_applied(&self, _config: &DatabaseConfig) -> Result<Vec<AppliedMigration>, DomainError> {
        Ok(self.rows.clone())
    }

    fn record_applied(
        &self,
        _config: &DatabaseConfig,
        _name: &str,
        _checksum: &str,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    fn remove_applied(&self, _config: &DatabaseConfig, _name: &str) -> Result<(), DomainError> {
        Ok(())
    }
}

#[test]
fn run_push_succeeds_with_stub_executor() {
    use renderers::SurrealDbRenderer;
    use schema_core::ports::{DatabaseExecutor, SchemaRenderer, SchemaSource};
    use schema_core::DbPushUseCase;

    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");

    let mut context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        temp.path().join("migrations").to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");

    let renderer: Arc<dyn SchemaRenderer> = Arc::new(SurrealDbRenderer::new());
    let database: Arc<dyn DatabaseExecutor> = Arc::new(OkExecutor);
    context.db_push = DbPushUseCase::new(context.schema_source.clone(), renderer, database);

    db::run_push(&context, &Printer::new()).expect("push");
}

#[test]
fn init_reports_schema_created() {
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("fresh.schema");
    let migrations = temp.path().join("migrations");

    let mut context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        migrations.to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");
    context.init_project = InitProjectUseCase::new(context.filesystem.clone());

    // schema file must not exist yet
    assert!(!schema_path.exists());
    cli::commands::init::run(&context, &Printer::new()).expect("init");
    assert!(schema_path.exists());
}

#[test]
fn migrate_status_lists_applied_and_pending_with_snapshot() {
    use cli::adapters::MigrationStoreAdapter;
    use schema_core::ports::MigrationStore;

    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");
    let migrations = temp.path().join("migrations");
    std::fs::create_dir_all(migrations.join("001_a")).expect("001");
    std::fs::create_dir_all(migrations.join("002_b")).expect("002");
    std::fs::write(
        migrations.join("001_a/snapshot.json"),
        serde_json::to_string(&sample_pulled_schema()).expect("json"),
    )
    .expect("snapshot");

    let mut context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        migrations.to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");

    let store = Arc::new(MigrationStoreAdapter::new(
        context.filesystem.clone(),
        migrations.to_string_lossy().into_owned(),
    ));
    // Touch list so store sees dirs.
    assert_eq!(store.list_migrations().expect("list").len(), 2);

    context.migrate_status = MigrateStatusUseCase::new(
        store,
        Arc::new(StatusLedger {
            rows: vec![AppliedMigration {
                name: "001_a".into(),
                applied_at: Some("2026-01-01T00:00:00Z".into()),
                checksum: "x".into(),
            }],
        }),
    );

    migrate::run_status(&context, &Printer::new()).expect("status");
}

#[test]
fn migrate_apply_and_rollback_success_messages() {
    use cli::adapters::MigrationStoreAdapter;
    use std::sync::Mutex;

    let temp = tempfile::tempdir().expect("tempdir");
    let schema_path = temp.path().join("awesome.schema");
    std::fs::write(&schema_path, MINIMAL_SCHEMA).expect("write");
    let migrations = temp.path().join("migrations");
    let mig = migrations.join("001_init");
    std::fs::create_dir_all(&mig).expect("mig");
    std::fs::write(mig.join("migration.surql"), "DEFINE TABLE user SCHEMAFULL;\n").expect("up");
    std::fs::write(mig.join("migration.down.surql"), "REMOVE TABLE user;\n").expect("down");

    struct RecordingLedger {
        applied: Mutex<Vec<AppliedMigration>>,
    }

    impl MigrationLedger for RecordingLedger {
        fn ensure_schema(&self, _config: &DatabaseConfig) -> Result<(), DomainError> {
            Ok(())
        }

        fn list_applied(
            &self,
            _config: &DatabaseConfig,
        ) -> Result<Vec<AppliedMigration>, DomainError> {
            Ok(self.applied.lock().expect("lock").clone())
        }

        fn record_applied(
            &self,
            _config: &DatabaseConfig,
            name: &str,
            checksum: &str,
        ) -> Result<(), DomainError> {
            self.applied.lock().expect("lock").push(AppliedMigration {
                name: name.to_owned(),
                applied_at: Some("now".into()),
                checksum: checksum.to_owned(),
            });
            Ok(())
        }

        fn remove_applied(&self, _config: &DatabaseConfig, name: &str) -> Result<(), DomainError> {
            self.applied.lock().expect("lock").retain(|row| row.name != name);
            Ok(())
        }
    }

    let mut context = test_context_with_introspector(
        schema_path.to_string_lossy().into_owned(),
        migrations.to_string_lossy().into_owned(),
        Arc::new(StubIntrospector {
            schema: sample_pulled_schema(),
        }),
    )
    .expect("context");

    let store = Arc::new(MigrationStoreAdapter::new(
        context.filesystem.clone(),
        migrations.to_string_lossy().into_owned(),
    ));
    let ledger = Arc::new(RecordingLedger {
        applied: Mutex::new(Vec::new()),
    });
    let database: Arc<dyn schema_core::ports::DatabaseExecutor> = Arc::new(OkExecutor);

    context.migrate_apply = MigrateApplyUseCase::new(
        store.clone(),
        context.filesystem.clone(),
        database.clone(),
        ledger.clone(),
    );
    context.migrate_rollback = MigrateRollbackUseCase::new(
        store,
        context.filesystem.clone(),
        database,
        ledger,
    );

    migrate::run_apply(&context, &Printer::new()).expect("apply");
    migrate::run_rollback(&context, 1, &Printer::new()).expect("rollback");
}
