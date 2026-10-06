# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 126 files · ~48,958 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1095 nodes · 2466 edges · 60 communities (48 shown, 12 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 112 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4e08aef9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- DomainError
- naming.rs
- Rust Best Practices
- AppContext
- core
- Post-Plan Work on main
- dispatch.rs
- cli crate
- surrealdb_workflow.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- SchemaIntrospector
- FileSystemPort
- src/print.rs
- validation.rs
- FieldType
- DatabaseConfig
- parser/src/lib.rs
- Performance Mindset
- cli
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- Field
- DatabaseSchema
- templates.rs
- check.sh
- install-hooks.sh
- MigrationOperation
- btreemap
- NamingConvention
- NamingCase
- arc
- cargo clippy
- Understanding Pointers
- DatabaseExecutor
- Milestone 1 Offline Schema Toolchain
- Error Handling
- Comments versus Documentation
- SurrealDB-First Policy
- Split-Core Hexagonal CLI Architecture
- migrate_apply.rs
- core crate
- migrate_dev.rs
- MigrationStore
- SchemaSource
- domain.rs
- Status Check 2026-10-06
- format_schema.rs
- Client Generators

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 152 edges
2. `DatabaseSchema` - 70 edges
3. `AppContext` - 39 edges
4. `FieldType` - 34 edges
5. `FileSystemPort` - 32 edges
6. `Printer` - 27 edges
7. `MigrationOperation` - 27 edges
8. `Parser` - 27 edges
9. `DatabaseConfig` - 26 edges
10. `Token` - 26 edges

## Surprising Connections (you probably didn't know these)
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `awesome-schema binary` --conceptually_related_to--> `cli crate`  [INFERRED]
  .agents/skills/split-core-hexagonal-cli/references/RUST.md → README.md
- `db pull` --conceptually_related_to--> `SchemaIntrospector`  [INFERRED]
  docs/roadmap/next.md → README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Awesome Schema Provider Ports** — readme_schema_renderer, readme_migration_renderer, readme_schema_introspector, readme_migration_store, readme_schema_source [EXTRACTED 1.00]
- **Faithful SurrealDB Schema Loop** — docs_roadmap_next_surrealdb_first, docs_roadmap_next_graph_edges, docs_roadmap_next_migration_lifecycle, docs_roadmap_next_db_pull, docs_roadmap_next_index_kinds [EXTRACTED 1.00]
- **Library and Binary Error Strategy** — agents_skills_rust_best_practices_references_chapter_04_result_over_panic, agents_skills_rust_best_practices_references_chapter_04_thiserror, agents_skills_rust_best_practices_references_chapter_04_anyhow, agents_skills_rust_best_practices_references_chapter_04_question_mark [EXTRACTED 1.00]
- **Rust Test Layers** — agents_skills_rust_best_practices_references_chapter_05_unit_tests, agents_skills_rust_best_practices_references_chapter_05_integration_tests, agents_skills_rust_best_practices_references_chapter_05_doc_tests [EXTRACTED 1.00]
- **Split-Core Architectural Layers** — agents_skills_split_core_hexagonal_cli_references_reference_domain_core, agents_skills_split_core_hexagonal_cli_references_reference_application_core, agents_skills_split_core_hexagonal_cli_references_reference_infrastructure_layer, agents_skills_split_core_hexagonal_cli_references_reference_shared_capability [EXTRACTED 1.00]
- **Static and Dynamic Dispatch Trade-off** — agents_skills_rust_best_practices_references_chapter_06_generics, agents_skills_rust_best_practices_references_chapter_06_static_dispatch, agents_skills_rust_best_practices_references_chapter_06_dynamic_dispatch [EXTRACTED 1.00]

## Communities (60 total, 12 thin omitted)

### Community 0 - "DomainError"
Cohesion: 0.11
Nodes (20): FsAdapter, DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError, RenderError (+12 more)

### Community 1 - "naming.rs"
Cohesion: 0.20
Nodes (8): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case(), to_snake_case()

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "AppContext"
Cohesion: 0.05
Nodes (28): list_schema_files(), ensure_can_write(), run_pull(), run_push(), write_single_schema(), write_split_schema(), run(), run() (+20 more)

### Community 4 - "core"
Cohesion: 0.07
Nodes (19): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+11 more)

### Community 5 - "Post-Plan Work on main"
Cohesion: 0.29
Nodes (10): Post-Plan Work on main, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, Awesome Schema DSL, Field Assignments, Flexible Object (+2 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (36): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+28 more)

### Community 7 - "cli crate"
Cohesion: 0.22
Nodes (10): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, anyhow, AppContext, AppContext, clap, cli crate (+2 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.10
Nodes (15): connect(), MINIMAL_SCHEMA, ws_connection_address(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_and_pull_preserves_relation_edge(), cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql() (+7 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.14
Nodes (20): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+12 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.27
Nodes (18): creates_initial_migration_from_empty_snapshot(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions(), detects_dropped_index(), detects_dropped_table() (+10 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.22
Nodes (23): NamingContext, preserves_field_name_when_fields_naming_not_set(), render_define_field(), render_define_index(), render_edge_schema(), render_model_schema(), render_operation(), render_unique_index() (+15 more)

### Community 15 - "SchemaIntrospector"
Cohesion: 0.15
Nodes (8): introspect_async(), SurrealDbIntrospector, StubIntrospector, merge_pulled_schema(), SchemaIntrospector, DbPullInput, DbPullOutput, DbPullUseCase

### Community 16 - "FileSystemPort"
Cohesion: 0.13
Nodes (5): MigrationStoreAdapter, FileSystemPort, InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 17 - "src/print.rs"
Cohesion: 0.22
Nodes (16): Generator, field_type_for_print(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field(), print_field_type() (+8 more)

### Community 18 - "validation.rs"
Cohesion: 0.32
Nodes (16): accepts_relation_field_naming_existing_edge(), rejects_edge_with_unknown_endpoint_model(), rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field() (+8 more)

### Community 19 - "FieldType"
Cohesion: 0.15
Nodes (15): FieldType, Array, Bool, Custom, Datetime, Float, Int, Model (+7 more)

### Community 20 - "DatabaseConfig"
Cohesion: 0.06
Nodes (26): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE (+18 more)

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.21
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "introspect.rs"
Cohesion: 0.18
Nodes (19): Index, edge_name_for_table(), is_relation_table(), map_database_info(), map_indexes(), maps_user_table_from_fixture(), normalize(), parse_index_define() (+11 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.07
Nodes (30): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, resolveExitCode, User (+22 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "Field"
Cohesion: 0.25
Nodes (11): Field, ObjectTypeDefinition, ObjectTypeField, expands_object_type_reference_into_nested_fields(), nested_fields_from_object_body(), nested_fields_from_object_type(), normalize_model_object_types(), normalize_schema() (+3 more)

### Community 28 - "DatabaseSchema"
Cohesion: 0.21
Nodes (7): DatabaseSchema, MemoryMigrationStore, StaticSchemaSource, map_fields(), map_record_links(), model_name_for_table(), parse_field_define()

### Community 36 - "MigrationOperation"
Cohesion: 0.10
Nodes (23): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+15 more)

### Community 38 - "NamingConvention"
Cohesion: 0.31
Nodes (9): MigrationPlan, Edge, NamingConvention, diff_edge(), diff_model(), field_operations_for_edge(), field_operations_for_model(), index_operations_for_model() (+1 more)

### Community 39 - "NamingCase"
Cohesion: 0.12
Nodes (10): map_field_path(), mapped_name(), NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase (+2 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "DatabaseExecutor"
Cohesion: 0.23
Nodes (5): DatabaseExecutor, SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 44 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.29
Nodes (8): Milestone 1 Offline Schema Toolchain, Roadmap Notes, SurrealDB Rust SDK Upgrade, SurrealDB 3.3.0, Awesome Schema, Prisma, SurrealDB, TypeORM

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "SurrealDB-First Policy"
Cohesion: 0.20
Nodes (10): db pull, Real Graph Edges, Index Kinds, SchemaDiffer, edge Block Gap, Index Flag Gap, @relation Gap, HNSW Vector Index (+2 more)

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.23
Nodes (10): CLI Architecture Checklist, CLI Architecture Reference, awesome-schema binary, Cargo Workspace Layout, DomainError, Rust CLI Examples, Rust Implementation Guide, thiserror (+2 more)

### Community 49 - "migrate_apply.rs"
Cohesion: 0.48
Nodes (4): checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase

### Community 50 - "core crate"
Cohesion: 0.26
Nodes (11): Later Backlog, cli crate, Connectivity Stubs, core crate, MigrationRenderer, MigrationStore, Planned Database Providers, renderers crate (+3 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.22
Nodes (6): MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort, StaticDiff

### Community 54 - "MigrationStore"
Cohesion: 0.18
Nodes (8): MigrationStore, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 55 - "SchemaSource"
Cohesion: 0.16
Nodes (7): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 58 - "domain.rs"
Cohesion: 0.15
Nodes (3): MigrateRollbackInput, MigrateRollbackOutput, MigrateRollbackUseCase

### Community 60 - "Status Check 2026-10-06"
Cohesion: 0.36
Nodes (9): Migration Lifecycle, db push, DropIndex Gap, migrate apply Gap, README Drift, Status Check 2026-10-06, MigrationOperation, Migration Workflow (+1 more)

### Community 61 - "format_schema.rs"
Cohesion: 0.52
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 64 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

## Knowledge Gaps
- **117 isolated node(s):** `LEDGER_TABLE`, `ENSURE_SCHEMA_SCRIPT`, `Init`, `Validate`, `Format` (+112 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 270 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `AppContext`, `core`, `surrealdb_workflow.rs`, `usecases.rs`, `surrealdb/mod.rs`, `SchemaIntrospector`, `FileSystemPort`, `validation.rs`, `FieldType`, `DatabaseConfig`, `parser/src/lib.rs`, `introspect.rs`, `Field`, `DatabaseSchema`, `arc`, `DatabaseExecutor`, `migrate_apply.rs`, `migrate_dev.rs`, `MigrationStore`, `SchemaSource`, `domain.rs`, `format_schema.rs`?**
  _High betweenness centrality (0.255) - this node is a cross-community bridge._
- **What connects `LEDGER_TABLE`, `ENSURE_SCHEMA_SCRIPT`, `Init` to the rest of the system?**
  _117 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DomainError` be split into smaller, more focused modules?**
  _Cohesion score 0.10621468926553672 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `DomainError`, `AppContext`, `core`, `usecases.rs`, `migrations/src/lib.rs`, `surrealdb/mod.rs`, `SchemaIntrospector`, `FileSystemPort`, `src/print.rs`, `validation.rs`, `DatabaseConfig`, `parser/src/lib.rs`, `introspect.rs`, `Field`, `btreemap`, `NamingConvention`, `arc`, `DatabaseExecutor`, `migrate_dev.rs`, `MigrationStore`, `SchemaSource`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._
- **Should `AppContext` be split into smaller, more focused modules?**
  _Cohesion score 0.0505175983436853 - nodes in this community are weakly interconnected._
- **Should `core` be split into smaller, more focused modules?**
  _Cohesion score 0.07308970099667775 - nodes in this community are weakly interconnected._
- **Should `dispatch.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05551020408163265 - nodes in this community are weakly interconnected._