use std::sync::Arc;

use codegen_rust::RustGenerator;
use codegen_typescript::TypeScriptGenerator;
use migrations::SchemaDiffer;
use renderers::SurrealDbRenderer;
use schema_core::ports::{
    DatabaseExecutor, FileSystemPort, MigrationRenderer, MigrationStore, SchemaIntrospector,
    SchemaRenderer, SchemaSource,
};
use schema_core::usecases::{CodeGeneratorPort, SchemaDiffPort};
use schema_core::{
    DatabaseSchema, DbPullUseCase, DbPushUseCase, DomainError, FormatSchemaUseCase,
    GenerateCodeUseCase, InitProjectUseCase, MigrateApplyUseCase, MigrateCreateUseCase,
    MigrateDevUseCase, MigrateStatusUseCase, ValidateSchemaUseCase,
};

use crate::adapters::{
    FsAdapter, MigrationStoreAdapter, SchemaFileSource, SurrealDbExecutor, SurrealDbIntrospector,
};

pub struct AppContext {
    pub schema_source: Arc<dyn SchemaSource>,
    pub init_project: InitProjectUseCase,
    pub validate_schema: ValidateSchemaUseCase,
    pub format_schema: FormatSchemaUseCase,
    pub generate_code: GenerateCodeUseCase,
    pub migrate_dev: MigrateDevUseCase,
    pub migrate_create: MigrateCreateUseCase,
    pub migrate_status: MigrateStatusUseCase,
    pub migrate_apply: MigrateApplyUseCase,
    pub db_pull: DbPullUseCase,
    pub db_push: DbPushUseCase,
    pub schema_path: String,
    pub migrations_dir: String,
    pub filesystem: Arc<dyn FileSystemPort>,
}

impl AppContext {
    pub fn with_paths(schema_path: String, migrations_dir: String) -> anyhow::Result<Self> {
        let filesystem: Arc<dyn FileSystemPort> = Arc::new(FsAdapter);
        build_context(filesystem, schema_path, migrations_dir)
    }

    pub fn load_schema(&self) -> Result<DatabaseSchema, DomainError> {
        self.schema_source.load_schema()
    }
}

#[doc(hidden)]
pub fn test_context_with_introspector(
    schema_path: String,
    migrations_dir: String,
    introspector: Arc<dyn SchemaIntrospector>,
) -> anyhow::Result<AppContext> {
    build_context_with_introspector(Arc::new(FsAdapter), schema_path, migrations_dir, introspector)
}

fn build_context(
    filesystem: Arc<dyn FileSystemPort>,
    schema_path: String,
    migrations_dir: String,
) -> anyhow::Result<AppContext> {
    build_context_with_introspector(
        filesystem,
        schema_path,
        migrations_dir,
        Arc::new(SurrealDbIntrospector),
    )
}

fn build_context_with_introspector(
    filesystem: Arc<dyn FileSystemPort>,
    schema_path: String,
    migrations_dir: String,
    introspector: Arc<dyn SchemaIntrospector>,
) -> anyhow::Result<AppContext> {
    let schema_source: Arc<dyn SchemaSource> =
        Arc::new(SchemaFileSource::new(filesystem.clone(), schema_path.clone()));

    let schema_renderer: Arc<dyn SchemaRenderer> = Arc::new(SurrealDbRenderer::new());
    let migration_renderer: Arc<dyn MigrationRenderer> = Arc::new(SurrealDbRenderer::new());
    let migration_store: Arc<dyn MigrationStore> =
        Arc::new(MigrationStoreAdapter::new(filesystem.clone(), migrations_dir.clone()));
    let diff: Arc<dyn SchemaDiffPort> = Arc::new(SchemaDiffer::new());
    let rust_generator: Arc<dyn CodeGeneratorPort> = Arc::new(RustGenerator::new());
    let typescript_generator: Arc<dyn CodeGeneratorPort> = Arc::new(TypeScriptGenerator::new());
    let introspector: Arc<dyn SchemaIntrospector> = introspector;
    let database: Arc<dyn DatabaseExecutor> = Arc::new(SurrealDbExecutor);

    Ok(AppContext {
        schema_source: schema_source.clone(),
        init_project: InitProjectUseCase::new(filesystem.clone()),
        validate_schema: ValidateSchemaUseCase::new(schema_source.clone()),
        format_schema: FormatSchemaUseCase::new(
            schema_source.clone(),
            filesystem.clone(),
            schema_path.clone(),
        ),
        generate_code: GenerateCodeUseCase::new(
            schema_source.clone(),
            schema_renderer.clone(),
            rust_generator,
            typescript_generator,
        ),
        migrate_dev: MigrateDevUseCase::new(
            schema_source.clone(),
            migration_store.clone(),
            migration_renderer.clone(),
            filesystem.clone(),
            diff,
        ),
        migrate_create: MigrateCreateUseCase::new(
            schema_source.clone(),
            migration_store.clone(),
            filesystem.clone(),
        ),
        migrate_status: MigrateStatusUseCase::new(migration_store.clone()),
        migrate_apply: MigrateApplyUseCase::new(
            migration_store,
            filesystem.clone(),
            database.clone(),
        ),
        db_pull: DbPullUseCase::new(introspector),
        db_push: DbPushUseCase::new(schema_source, schema_renderer, database),
        schema_path,
        migrations_dir,
        filesystem,
    })
}
