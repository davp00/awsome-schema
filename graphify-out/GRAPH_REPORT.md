# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 123 files · ~46,085 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1091 nodes · 2236 edges · 68 communities (50 shown, 18 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 111 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b968f449`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- DomainError
- naming.rs
- Type State Pattern
- Printer
- core
- Status Check 2026-10-06
- dispatch.rs
- cli crate
- surrealdb_workflow.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- AppContext
- FileSystemPort
- src/print.rs
- validation.rs
- Infrastructure Layer
- database_config.rs
- parser/src/lib.rs
- Performance Mindset
- cli
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- db.rs
- schema_core
- templates.rs
- check.sh
- install-hooks.sh
- MigrationOperation
- btreemap
- DatabaseSchema
- NamingCase
- arc
- cargo clippy
- Understanding Pointers
- DomainError
- cli/src/lib.rs
- Error Handling
- Rust Best Practices
- ImportUsersUseCase
- Split-Core Hexagonal CLI Architecture
- Coding Styles and Idioms
- ImportUsersUseCase
- migrate_dev.rs
- MigrationStore
- SchemaSource
- RecordingRenderer
- FsAdapter
- surrealdb_executor.rs
- introspector.rs
- di.rs
- DatabaseExecutor
- usecases/mod.rs
- migrate_create.rs
- migrate_apply.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 81 edges
2. `DatabaseSchema` - 70 edges
3. `AppContext` - 37 edges
4. `FileSystemPort` - 30 edges
5. `FieldType` - 28 edges
6. `Parser` - 27 edges
7. `Token` - 26 edges
8. `Printer` - 26 edges
9. `MigrationOperation` - 22 edges
10. `Rust Best Practices` - 18 edges

## Surprising Connections (you probably didn't know these)
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `awesome-schema binary` --conceptually_related_to--> `cli crate`  [INFERRED]
  .agents/skills/split-core-hexagonal-cli/references/RUST.md → README.md
- `COMPUTED Clause` --conceptually_related_to--> `Field Assignments`  [INFERRED]
  docs/roadmap/surrealdb-3.3.md → README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Awesome Schema Provider Ports** — readme_schema_renderer, readme_migration_renderer, readme_schema_introspector, readme_migration_store, readme_schema_source [EXTRACTED 1.00]
- **Faithful SurrealDB Schema Loop** — docs_roadmap_next_surrealdb_first, docs_roadmap_next_graph_edges, docs_roadmap_next_migration_lifecycle, docs_roadmap_next_db_pull, docs_roadmap_next_index_kinds [EXTRACTED 1.00]
- **Library and Binary Error Strategy** — agents_skills_rust_best_practices_references_chapter_04_result_over_panic, agents_skills_rust_best_practices_references_chapter_04_thiserror, agents_skills_rust_best_practices_references_chapter_04_anyhow, agents_skills_rust_best_practices_references_chapter_04_question_mark [EXTRACTED 1.00]
- **Rust Test Layers** — agents_skills_rust_best_practices_references_chapter_05_unit_tests, agents_skills_rust_best_practices_references_chapter_05_integration_tests, agents_skills_rust_best_practices_references_chapter_05_doc_tests [EXTRACTED 1.00]
- **Split-Core Architectural Layers** — agents_skills_split_core_hexagonal_cli_references_reference_domain_core, agents_skills_split_core_hexagonal_cli_references_reference_application_core, agents_skills_split_core_hexagonal_cli_references_reference_infrastructure_layer, agents_skills_split_core_hexagonal_cli_references_reference_shared_capability [EXTRACTED 1.00]
- **Static and Dynamic Dispatch Trade-off** — agents_skills_rust_best_practices_references_chapter_06_generics, agents_skills_rust_best_practices_references_chapter_06_static_dispatch, agents_skills_rust_best_practices_references_chapter_06_dynamic_dispatch [EXTRACTED 1.00]

## Communities (68 total, 18 thin omitted)

### Community 0 - "DomainError"
Cohesion: 0.08
Nodes (40): Field, FieldType, Array, Bool, Custom, Datetime, Float, Int (+32 more)

### Community 1 - "naming.rs"
Cohesion: 0.18
Nodes (9): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingContext, split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case() (+1 more)

### Community 2 - "Type State Pattern"
Cohesion: 0.21
Nodes (5): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State

### Community 3 - "Printer"
Cohesion: 0.12
Nodes (6): run_push(), run(), run(), dispatch(), run(), Printer

### Community 4 - "core"
Cohesion: 0.06
Nodes (22): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+14 more)

### Community 5 - "Status Check 2026-10-06"
Cohesion: 0.06
Nodes (53): Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain (+45 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (34): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+26 more)

### Community 7 - "cli crate"
Cohesion: 0.24
Nodes (9): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, AppContext, clap, cli crate, core crate (+1 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.08
Nodes (14): connect(), MINIMAL_SCHEMA, ws_connection_address(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql(), cli_init_creates_project_files() (+6 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.27
Nodes (15): db_push_renders_and_applies_schema(), format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), migrate_apply_executes_pending_migration_scripts(), migrate_apply_skips_missing_and_empty_scripts() (+7 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.24
Nodes (14): NamingConvention, creates_initial_migration_from_empty_snapshot(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_dropped_table(), diff_model(), diff_schemas() (+6 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (22): preserves_field_name_when_fields_naming_not_set(), render_define_field(), render_define_index(), render_define_table(), render_edge_schema(), render_model_schema(), render_operation(), render_unique_index() (+14 more)

### Community 15 - "AppContext"
Cohesion: 0.15
Nodes (5): run_apply(), run_create(), run_dev(), run_status(), AppContext

### Community 16 - "FileSystemPort"
Cohesion: 0.09
Nodes (12): MigrationStoreAdapter, list_schema_files(), load_directory_schema(), SchemaFileSource, FileSystemPort, FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase (+4 more)

### Community 17 - "src/print.rs"
Cohesion: 0.13
Nodes (17): Generator, field_type_for_print(), naming_case_str(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field() (+9 more)

### Community 18 - "validation.rs"
Cohesion: 0.38
Nodes (13): rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field(), rejects_nested_field_without_parent_object(), rejects_wrong_id_field_type() (+5 more)

### Community 19 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 20 - "database_config.rs"
Cohesion: 0.31
Nodes (8): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value(), resolves_env_reference()

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.19
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.23
Nodes (5): cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "introspect.rs"
Cohesion: 0.17
Nodes (23): edge_name_for_table(), is_relation_table(), map_database_info(), map_fields(), map_indexes(), map_record_links(), map_scalar_type_name(), maps_user_table_from_fixture() (+15 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.26
Nodes (12): UserDatabaseRepository, UserRepository, AppContext, Cli, DomainError, FileSystemPort, ImportUsersCommand, ImportUsersInput (+4 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "db.rs"
Cohesion: 0.26
Nodes (9): ensure_can_write(), run_pull(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir(), table_fragment_path(), TABLE_SUFFIX (+1 more)

### Community 36 - "MigrationOperation"
Cohesion: 0.13
Nodes (15): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+7 more)

### Community 37 - "btreemap"
Cohesion: 0.18
Nodes (5): Edge, TableMode, Schemafull, Schemaless, Relation

### Community 38 - "DatabaseSchema"
Cohesion: 0.11
Nodes (5): DatabaseSchema, MemoryFs, MemoryMigrationStore, RecordingDatabase, StaticSchemaSource

### Community 39 - "NamingCase"
Cohesion: 0.12
Nodes (11): Index, Model, map_field_path(), mapped_name(), NamingCase, CamelCase, KebabCase, Lowercase (+3 more)

### Community 40 - "arc"
Cohesion: 0.15
Nodes (6): StubIntrospector, merge_pulled_schema(), SchemaIntrospector, DbPullInput, DbPullOutput, DbPullUseCase

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "DomainError"
Cohesion: 0.18
Nodes (11): resolveExitCode, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError, resolve_exit_code, thiserror (+3 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Rust Best Practices"
Cohesion: 0.33
Nodes (7): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 47 - "ImportUsersUseCase"
Cohesion: 0.25
Nodes (8): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, OutputWriter

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 50 - "ImportUsersUseCase"
Cohesion: 0.50
Nodes (3): ImportUsersUseCase, mockall, UserRepository

### Community 51 - "migrate_dev.rs"
Cohesion: 0.30
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "MigrationStore"
Cohesion: 0.25
Nodes (4): MigrationStore, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase

### Community 55 - "SchemaSource"
Cohesion: 0.25
Nodes (4): SchemaSource, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "RecordingRenderer"
Cohesion: 0.20
Nodes (3): EmptyDiff, RecordingRenderer, StaticDiff

### Community 59 - "surrealdb_executor.rs"
Cohesion: 0.36
Nodes (4): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor

### Community 62 - "di.rs"
Cohesion: 0.38
Nodes (3): build_context(), build_context_with_introspector(), test_context_with_introspector()

### Community 63 - "DatabaseExecutor"
Cohesion: 0.38
Nodes (3): DatabaseConfig, DatabaseExecutor, DbPushUseCase

### Community 64 - "usecases/mod.rs"
Cohesion: 0.33
Nodes (3): Datasource, DbPushInput, DbPushOutput

### Community 65 - "migrate_create.rs"
Cohesion: 0.47
Nodes (3): MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase

### Community 66 - "migrate_apply.rs"
Cohesion: 0.60
Nodes (3): MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase

## Knowledge Gaps
- **111 isolated node(s):** `MINIMAL_SCHEMA`, `Lexer`, `Relation`, `Array`, `Bool` (+106 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 301 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DatabaseSchema` connect `DatabaseSchema` to `DomainError`, `core`, `usecases.rs`, `migrations/src/lib.rs`, `surrealdb/mod.rs`, `AppContext`, `FileSystemPort`, `src/print.rs`, `validation.rs`, `parser/src/lib.rs`, `introspect.rs`, `db.rs`, `btreemap`, `arc`, `migrate_dev.rs`, `MigrationStore`, `SchemaSource`, `RecordingRenderer`, `introspector.rs`, `usecases/mod.rs`?**
  _High betweenness centrality (0.174) - this node is a cross-community bridge._
- **What connects `MINIMAL_SCHEMA`, `Lexer`, `Relation` to the rest of the system?**
  _111 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DomainError` be split into smaller, more focused modules?**
  _Cohesion score 0.07894736842105263 - nodes in this community are weakly interconnected._
- **Why does `DomainError` connect `DomainError` to `usecases/mod.rs`, `migrate_create.rs`, `migrate_apply.rs`, `core`, `FileSystemPort`, `validation.rs`, `migrate_dev.rs`, `database_config.rs`, `MigrationStore`, `SchemaSource`, `DatabaseExecutor`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Should `Printer` be split into smaller, more focused modules?**
  _Cohesion score 0.11857707509881422 - nodes in this community are weakly interconnected._
- **Why does `FileSystemPort` connect `FileSystemPort` to `migrate_create.rs`, `migrate_apply.rs`, `DatabaseSchema`, `AppContext`, `migrate_dev.rs`, `SchemaSource`, `FsAdapter`, `di.rs`, `DatabaseExecutor`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Should `core` be split into smaller, more focused modules?**
  _Cohesion score 0.06207482993197279 - nodes in this community are weakly interconnected._