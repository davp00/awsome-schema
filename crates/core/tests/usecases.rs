use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use core::domain::{
    DatabaseSchema, Datasource, Field, FieldType, MigrationOperation, MigrationPlan, Model,
    NamingConvention, TableMode,
};
use core::ports::{
    AppliedMigration, DatabaseExecutor, FileSystemPort, MigrationLedger, MigrationRenderer,
    MigrationStore, SchemaRenderer, SchemaSource,
};
use core::usecases::{
    CodeGeneratorPort, DbPushInput, DbPushUseCase, FormatSchemaInput, FormatSchemaUseCase,
    GenerateCodeInput, GenerateCodeTarget, GenerateCodeUseCase, InitProjectInput,
    InitProjectUseCase, MigrateApplyInput, MigrateApplyUseCase, MigrateCreateInput,
    MigrateCreateUseCase, MigrateDevInput, MigrateDevUseCase, MigrateRollbackInput,
    MigrateRollbackUseCase, MigrateStatusInput, MigrateStatusUseCase, MigrationApplyState,
    SchemaDiffPort, ValidateSchemaInput, ValidateSchemaUseCase,
};
use core::{DatabaseConfig, DomainError};

struct MemoryFs {
    files: Mutex<BTreeMap<String, String>>,
    dirs: Mutex<Vec<String>>,
}

impl MemoryFs {
    const fn new() -> Self {
        Self { files: Mutex::new(BTreeMap::new()), dirs: Mutex::new(Vec::new()) }
    }
}

impl FileSystemPort for MemoryFs {
    fn read_to_string(&self, path: &str) -> Result<String, DomainError> {
        self.files
            .lock()
            .expect("lock")
            .get(path)
            .cloned()
            .ok_or_else(|| DomainError::SchemaReadFailed(path.to_owned()))
    }

    fn write_string(&self, path: &str, content: &str) -> Result<(), DomainError> {
        self.files.lock().expect("lock").insert(path.to_owned(), content.to_owned());
        Ok(())
    }

    fn create_dir_all(&self, path: &str) -> Result<(), DomainError> {
        self.dirs.lock().expect("lock").push(path.to_owned());
        Ok(())
    }

    fn exists(&self, path: &str) -> bool {
        self.files.lock().expect("lock").contains_key(path)
            || self.dirs.lock().expect("lock").iter().any(|dir| dir == path)
    }

    fn is_directory(&self, path: &str) -> bool {
        self.dirs.lock().expect("lock").iter().any(|dir| dir == path)
    }

    fn list_dir(&self, _path: &str) -> Result<Vec<String>, DomainError> {
        Ok(Vec::new())
    }

    fn remove_file(&self, path: &str) -> Result<(), DomainError> {
        self.files.lock().expect("lock").remove(path);
        Ok(())
    }
}

struct StaticSchemaSource {
    schema: DatabaseSchema,
    raw: String,
}

impl SchemaSource for StaticSchemaSource {
    fn load_schema(&self) -> Result<DatabaseSchema, DomainError> {
        Ok(self.schema.clone())
    }

    fn load_raw(&self) -> Result<String, DomainError> {
        Ok(self.raw.clone())
    }
}

struct RecordingRenderer {
    output: String,
}

impl SchemaRenderer for RecordingRenderer {
    fn render_schema(&self, _schema: &DatabaseSchema) -> Result<String, DomainError> {
        Ok(self.output.clone())
    }
}

impl MigrationRenderer for RecordingRenderer {
    fn render_migration(&self, _plan: &MigrationPlan) -> Result<String, DomainError> {
        Ok("-- migration".to_owned())
    }
}

struct RecordingDatabase {
    scripts: Mutex<Vec<String>>,
}

impl DatabaseExecutor for RecordingDatabase {
    fn execute_script(&self, _config: &DatabaseConfig, script: &str) -> Result<(), DomainError> {
        self.scripts.lock().expect("lock").push(script.to_owned());
        Ok(())
    }
}

struct StaticGenerator {
    label: String,
}

impl CodeGeneratorPort for StaticGenerator {
    fn generate(&self, _schema: &DatabaseSchema) -> Result<String, DomainError> {
        Ok(self.label.clone())
    }
}

struct MemoryMigrationStore {
    migrations: Mutex<Vec<String>>,
    snapshot: Mutex<Option<DatabaseSchema>>,
}

impl MigrationStore for MemoryMigrationStore {
    fn load_last_snapshot(&self) -> Result<Option<DatabaseSchema>, DomainError> {
        Ok(self.snapshot.lock().expect("lock").clone())
    }

    fn save_snapshot(
        &self,
        schema: &DatabaseSchema,
        _migration_dir: &str,
    ) -> Result<(), DomainError> {
        *self.snapshot.lock().expect("lock") = Some(schema.clone());
        Ok(())
    }

    fn list_migrations(&self) -> Result<Vec<String>, DomainError> {
        Ok(self.migrations.lock().expect("lock").clone())
    }
}

struct MemoryLedger {
    applied: Mutex<Vec<AppliedMigration>>,
}

impl MemoryLedger {
    fn new() -> Self {
        Self { applied: Mutex::new(Vec::new()) }
    }

    fn with_applied(names: &[&str]) -> Self {
        Self {
            applied: Mutex::new(
                names
                    .iter()
                    .map(|name| AppliedMigration {
                        name: (*name).to_owned(),
                        applied_at: Some(format!("2026-01-01T00:00:00Z-{name}")),
                        checksum: "checksum".to_owned(),
                    })
                    .collect(),
            ),
        }
    }
}

impl MigrationLedger for MemoryLedger {
    fn ensure_schema(&self, _config: &DatabaseConfig) -> Result<(), DomainError> {
        Ok(())
    }

    fn list_applied(&self, _config: &DatabaseConfig) -> Result<Vec<AppliedMigration>, DomainError> {
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
            applied_at: Some("now".to_owned()),
            checksum: checksum.to_owned(),
        });
        Ok(())
    }

    fn remove_applied(&self, _config: &DatabaseConfig, name: &str) -> Result<(), DomainError> {
        self.applied.lock().expect("lock").retain(|row| row.name != name);
        Ok(())
    }
}

struct StaticDiff;

impl SchemaDiffPort for StaticDiff {
    fn diff(
        &self,
        _from: Option<&DatabaseSchema>,
        _to: &DatabaseSchema,
        name: &str,
    ) -> MigrationPlan {
        MigrationPlan {
            name: name.to_owned(),
            operations: vec![MigrationOperation::CreateTable {
                name: "user".to_owned(),
                mode: TableMode::Schemafull,
                relation: None,
            }],
            naming: NamingConvention::default(),
        }
    }
}

fn sample_schema() -> DatabaseSchema {
    DatabaseSchema {
        datasource: Datasource {
            provider: "surrealdb".to_owned(),
            url: Some("ws://127.0.0.1:8000".to_owned()),
            namespace: Some("test".to_owned()),
            database: Some("main".to_owned()),
            extra: BTreeMap::new(),
        },
        naming: NamingConvention::default(),
        generators: Vec::new(),
        object_types: Vec::new(),
        models: vec![Model {
            name: "User".to_owned(),
            fields: vec![Field {
                name: "id".to_owned(),
                field_type: FieldType::RecordId("User".to_owned()),
                optional: false,
                unique: false,
                is_id: true,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }],
        edges: Vec::new(),
    }
}

#[test]
fn init_project_creates_schema_and_migrations_dir() {
    let fs = Arc::new(MemoryFs::new());
    let use_case = InitProjectUseCase::new(fs.clone());
    let output = use_case
        .execute(InitProjectInput {
            schema_path: "awesome.schema".to_owned(),
            migrations_dir: "migrations".to_owned(),
        })
        .expect("init");

    assert!(output.schema_created);
    assert!(output.migrations_dir_created);
    assert!(fs.exists("awesome.schema"));
}

#[test]
fn init_project_skips_existing_schema() {
    let fs = Arc::new(MemoryFs::new());
    fs.write_string("awesome.schema", "existing").expect("seed");
    let use_case = InitProjectUseCase::new(fs);
    let output = use_case
        .execute(InitProjectInput {
            schema_path: "awesome.schema".to_owned(),
            migrations_dir: "migrations".to_owned(),
        })
        .expect("init");

    assert!(!output.schema_created);
}

#[test]
fn validate_schema_counts_models_and_edges() {
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema, raw: String::new() });
    let use_case = ValidateSchemaUseCase::new(source);
    let output = use_case.execute(ValidateSchemaInput).expect("validate");
    assert_eq!(output.model_count, 1);
    assert_eq!(output.edge_count, 0);
}

#[test]
fn format_schema_trims_trailing_blank_lines() {
    let schema = sample_schema();
    let source =
        Arc::new(StaticSchemaSource { schema, raw: "model User {\n  id @id\n}\n\n".to_owned() });
    let fs = Arc::new(MemoryFs::new());
    let use_case = FormatSchemaUseCase::new(source, fs, "awesome.schema".to_owned());
    let output = use_case.execute(FormatSchemaInput { write_back: true, schema_files: vec!["awesome.schema".into()] }).expect("format");

    assert!(output.written);
    assert_eq!(output.formatted, "model User {\n  id @id\n}\n");
}

#[test]
fn generate_code_targets_schema_rust_and_typescript() {
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema, raw: String::new() });
    let renderer = Arc::new(RecordingRenderer { output: "SQL".to_owned() });
    let use_case = GenerateCodeUseCase::new(
        source,
        renderer,
        Arc::new(StaticGenerator { label: "RUST".to_owned() }),
        Arc::new(StaticGenerator { label: "TS".to_owned() }),
    );

    assert_eq!(
        use_case
            .execute(GenerateCodeInput { target: GenerateCodeTarget::Schema })
            .expect("schema")
            .content,
        "SQL"
    );
    assert_eq!(
        use_case
            .execute(GenerateCodeInput { target: GenerateCodeTarget::Rust })
            .expect("rust")
            .content,
        "RUST"
    );
    assert_eq!(
        use_case
            .execute(GenerateCodeInput { target: GenerateCodeTarget::TypeScript })
            .expect("typescript")
            .content,
        "TS"
    );
}

#[test]
fn migrate_apply_executes_pending_migration_scripts() {
    let fs = Arc::new(MemoryFs::new());
    fs.write_string("migrations/001_init/migration.surql", "DEFINE TABLE user SCHEMAFULL;")
        .expect("write");

    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec!["001_init".to_owned()]),
        snapshot: Mutex::new(None),
    });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });
    let ledger = Arc::new(MemoryLedger::new());

    let use_case = MigrateApplyUseCase::new(store, fs, database.clone(), ledger.clone());
    let output = use_case
        .execute(MigrateApplyInput {
            migrations_dir: "migrations".to_owned(),
            datasource: sample_schema().datasource,
        })
        .expect("apply");

    assert_eq!(output.applied, 1);
    assert_eq!(output.skipped, 0);
    assert_eq!(database.scripts.lock().expect("lock").len(), 1);
    assert_eq!(ledger.list_applied(&DatabaseConfig::from_datasource(&sample_schema().datasource).unwrap()).unwrap().len(), 1);
}

#[test]
fn migrate_apply_skips_already_recorded_migrations() {
    let fs = Arc::new(MemoryFs::new());
    fs.write_string("migrations/001_init/migration.surql", "DEFINE TABLE user SCHEMAFULL;")
        .expect("write");
    fs.write_string("migrations/002_next/migration.surql", "DEFINE TABLE post SCHEMAFULL;")
        .expect("write");

    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec!["001_init".to_owned(), "002_next".to_owned()]),
        snapshot: Mutex::new(None),
    });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });
    let ledger = Arc::new(MemoryLedger::with_applied(&["001_init"]));

    let use_case = MigrateApplyUseCase::new(store, fs, database.clone(), ledger.clone());
    let output = use_case
        .execute(MigrateApplyInput {
            migrations_dir: "migrations".to_owned(),
            datasource: sample_schema().datasource,
        })
        .expect("apply");

    assert_eq!(output.applied, 1);
    assert_eq!(output.skipped, 1);
    assert_eq!(database.scripts.lock().expect("lock").len(), 1);
    assert_eq!(
        ledger
            .list_applied(
                &DatabaseConfig::from_datasource(&sample_schema().datasource).unwrap()
            )
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn migrate_rollback_runs_down_then_removes_ledger() {
    let fs = Arc::new(MemoryFs::new());
    fs.write_string("migrations/001_init/migration.down.surql", "REMOVE TABLE user;")
        .expect("write");

    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec!["001_init".to_owned()]),
        snapshot: Mutex::new(None),
    });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });
    let ledger = Arc::new(MemoryLedger::with_applied(&["001_init"]));

    let use_case = MigrateRollbackUseCase::new(store, fs, database.clone(), ledger.clone());
    let output = use_case
        .execute(MigrateRollbackInput {
            migrations_dir: "migrations".to_owned(),
            datasource: sample_schema().datasource,
            steps: 1,
        })
        .expect("rollback");

    assert_eq!(output.rolled_back, vec!["001_init".to_owned()]);
    assert_eq!(database.scripts.lock().expect("lock").as_slice(), ["REMOVE TABLE user;"]);
    assert!(
        ledger
            .list_applied(
                &DatabaseConfig::from_datasource(&sample_schema().datasource).unwrap()
            )
            .unwrap()
            .is_empty()
    );
}

#[test]
fn migrate_rollback_errors_when_down_missing() {
    let fs = Arc::new(MemoryFs::new());
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec!["001_init".to_owned()]),
        snapshot: Mutex::new(None),
    });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });
    let ledger = Arc::new(MemoryLedger::with_applied(&["001_init"]));

    let use_case = MigrateRollbackUseCase::new(store, fs, database, ledger.clone());
    let error = use_case
        .execute(MigrateRollbackInput {
            migrations_dir: "migrations".to_owned(),
            datasource: sample_schema().datasource,
            steps: 1,
        })
        .expect_err("missing down");

    assert!(matches!(error, DomainError::MigrationError(_)));
    assert_eq!(
        ledger
            .list_applied(
                &DatabaseConfig::from_datasource(&sample_schema().datasource).unwrap()
            )
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn db_push_renders_and_applies_schema() {
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema, raw: String::new() });
    let renderer = Arc::new(RecordingRenderer { output: "DEFINE TABLE user;".to_owned() });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });
    let use_case = DbPushUseCase::new(source, renderer, database.clone());
    let output =
        use_case.execute(DbPushInput { datasource: sample_schema().datasource }).expect("push");

    assert_eq!(output.statements, "DEFINE TABLE user;");
    assert_eq!(database.scripts.lock().expect("lock").len(), 1);
}

#[test]
fn migrate_status_reports_applied_and_pending() {
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec!["001".to_owned(), "002".to_owned()]),
        snapshot: Mutex::new(Some(sample_schema())),
    });
    let ledger = Arc::new(MemoryLedger::with_applied(&["001"]));
    let use_case = MigrateStatusUseCase::new(store, ledger);
    let output = use_case
        .execute(MigrateStatusInput { datasource: sample_schema().datasource })
        .expect("status");
    assert_eq!(output.migrations.len(), 2);
    assert_eq!(output.migrations[0].state, MigrationApplyState::Applied);
    assert_eq!(output.migrations[1].state, MigrationApplyState::Pending);
    assert_eq!(output.applied_count, 1);
    assert_eq!(output.pending_count, 1);
    assert!(output.has_snapshot);
}

#[test]
fn migrate_create_writes_empty_migration_files() {
    let fs = Arc::new(MemoryFs::new());
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema, raw: String::new() });
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(Vec::new()),
        snapshot: Mutex::new(None),
    });
    let use_case = MigrateCreateUseCase::new(source, store, fs);
    let output = use_case
        .execute(MigrateCreateInput {
            migrations_dir: "migrations".to_owned(),
            name: "manual".to_owned(),
        })
        .expect("create");

    assert!(output.migration_dir.starts_with("migrations/"));
    assert!(output.migration_dir.ends_with("_manual"));
}

#[test]
fn migrate_dev_creates_migration_when_diff_has_operations() {
    let fs = Arc::new(MemoryFs::new());
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema, raw: String::new() });
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(Vec::new()),
        snapshot: Mutex::new(None),
    });
    let renderer = Arc::new(RecordingRenderer { output: String::new() });
    let diff = Arc::new(StaticDiff);
    let use_case = MigrateDevUseCase::new(source, store, renderer, fs, diff);

    let output = use_case
        .execute(MigrateDevInput {
            migrations_dir: "migrations".to_owned(),
            migration_name: Some("create_user".to_owned()),
        })
        .expect("dev");

    assert!(output.created);
    assert_eq!(output.operation_count, 1);
    assert!(output.migration_dir.contains("create_user"));
}

struct EmptyDiff;

impl SchemaDiffPort for EmptyDiff {
    fn diff(
        &self,
        _from: Option<&DatabaseSchema>,
        _to: &DatabaseSchema,
        name: &str,
    ) -> MigrationPlan {
        MigrationPlan {
            name: name.to_owned(),
            operations: Vec::new(),
            naming: NamingConvention::default(),
        }
    }
}

#[test]
fn migrate_apply_skips_missing_and_empty_scripts() {
    let fs = Arc::new(MemoryFs::new());
    fs.write_string("migrations/001_init/migration.surql", "DEFINE TABLE user SCHEMAFULL;")
        .expect("write");
    fs.write_string("migrations/002_empty/migration.surql", "  \n").expect("write empty");

    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(vec![
            "001_init".to_owned(),
            "002_empty".to_owned(),
            "003_missing".to_owned(),
        ]),
        snapshot: Mutex::new(None),
    });
    let database = Arc::new(RecordingDatabase { scripts: Mutex::new(Vec::new()) });

    let use_case = MigrateApplyUseCase::new(store, fs, database.clone(), Arc::new(MemoryLedger::new()));
    let output = use_case
        .execute(MigrateApplyInput {
            migrations_dir: "migrations".to_owned(),
            datasource: sample_schema().datasource,
        })
        .expect("apply");

    assert_eq!(output.applied, 1);
    assert_eq!(output.skipped, 0);
    assert_eq!(database.scripts.lock().expect("lock").len(), 1);
}

#[test]
fn format_schema_without_write_back() {
    let schema = sample_schema();
    let source =
        Arc::new(StaticSchemaSource { schema, raw: "model User {\n  id @id\n}\n\n".to_owned() });
    let fs = Arc::new(MemoryFs::new());
    let use_case = FormatSchemaUseCase::new(source, fs, "awesome.schema".to_owned());
    let output = use_case.execute(FormatSchemaInput { write_back: false, schema_files: vec!["awesome.schema".into()] }).expect("format");

    assert!(!output.written);
    assert_eq!(output.formatted, "model User {\n  id @id\n}\n");
}

#[test]
fn migrate_dev_uses_previous_snapshot_for_down_migration() {
    let fs = Arc::new(MemoryFs::new());
    let previous = sample_schema();
    let mut current = sample_schema();
    current.models[0].fields.push(Field {
        name: "name".to_owned(),
        field_type: FieldType::String,
        optional: false,
        unique: false,
        is_id: false,
        default_value: None,
        default_always: false,
        value_expression: None,
        readonly: false,
        flexible: false,
        link_target: None,
        relation_name: None,
        attributes: BTreeMap::new(),
    });

    let source = Arc::new(StaticSchemaSource { schema: current.clone(), raw: String::new() });
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(Vec::new()),
        snapshot: Mutex::new(Some(previous)),
    });
    let renderer = Arc::new(RecordingRenderer { output: "UP".to_owned() });
    let diff = Arc::new(StaticDiff);
    let use_case = MigrateDevUseCase::new(source, store, renderer, fs.clone(), diff);

    let output = use_case
        .execute(MigrateDevInput {
            migrations_dir: "migrations".to_owned(),
            migration_name: Some("add_name".to_owned()),
        })
        .expect("dev");

    assert!(output.created);
    assert!(fs.exists(&format!("{}/migration.down.surql", output.migration_dir)));
}

#[test]
fn migrate_dev_reports_no_changes_when_diff_is_empty() {
    let fs = Arc::new(MemoryFs::new());
    let schema = sample_schema();
    let source = Arc::new(StaticSchemaSource { schema: schema.clone(), raw: String::new() });
    let store = Arc::new(MemoryMigrationStore {
        migrations: Mutex::new(Vec::new()),
        snapshot: Mutex::new(Some(schema)),
    });
    let renderer = Arc::new(RecordingRenderer { output: String::new() });
    let use_case = MigrateDevUseCase::new(source, store, renderer, fs, Arc::new(EmptyDiff));

    let output = use_case
        .execute(MigrateDevInput { migrations_dir: "migrations".to_owned(), migration_name: None })
        .expect("dev");

    assert!(!output.created);
    assert_eq!(output.operation_count, 0);
}
