# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 126 files · ~51,752 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1185 nodes · 2538 edges · 70 communities (49 shown, 21 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 112 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3babc027`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Parser
- MigrationOperation
- Rust Best Practices
- db.rs
- generate_code.rs
- AppContext
- dispatch.rs
- cli crate
- surrealdb_workflow.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- DatabaseSchema
- DomainError
- validation.rs
- mapped_name
- DatabaseConfig
- parser/src/lib.rs
- Performance Mindset
- cli
- FieldType
- ImportUsersUseCase
- Git Commit Skill
- Field
- database_config.rs
- templates.rs
- check.sh
- install-hooks.sh
- Printer
- btreemap
- DomainError
- naming.rs
- tests/db_pull.rs
- cargo clippy
- Understanding Pointers
- Infrastructure Layer
- cli/src/lib.rs
- Error Handling
- Comments versus Documentation
- model.rs
- Split-Core Hexagonal CLI Architecture
- arc
- Status Check 2026-10-06
- migrate_dev.rs
- ImportUsersUseCase
- SchemaSource
- schema_core
- ImportUsersUseCase
- format.rs
- format_schema.rs
- MigrationStore
- domain.rs
- migrate_status.rs
- NamingCase
- surrealdb_executor.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 91 edges
2. `AppContext` - 39 edges
3. `FieldType` - 35 edges
4. `DatabaseSchema` - 35 edges
5. `FileSystemPort` - 31 edges
6. `Parser` - 27 edges
7. `Printer` - 27 edges
8. `Field` - 26 edges
9. `Model` - 21 edges
10. `NamingConvention` - 21 edges

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

## Communities (70 total, 21 thin omitted)

### Community 0 - "Parser"
Cohesion: 0.21
Nodes (6): link_target_from_type(), parse_index_fields(), parse_schema(), ParsedFieldAttributes, Parser, validate_field_rules()

### Community 1 - "MigrationOperation"
Cohesion: 0.12
Nodes (16): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+8 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "db.rs"
Cohesion: 0.26
Nodes (9): ensure_can_write(), run_pull(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir(), table_fragment_path(), TABLE_SUFFIX (+1 more)

### Community 4 - "generate_code.rs"
Cohesion: 0.11
Nodes (13): map_field_type(), RustGenerator, CodeGenerator, map_field_type(), TypeScriptGenerator, CodeGeneratorPort, GenerateCodeInput, GenerateCodeOutput (+5 more)

### Community 5 - "AppContext"
Cohesion: 0.24
Nodes (9): run_apply(), run_create(), run_dev(), run_rollback(), run_status(), AppContext, build_context(), build_context_with_introspector() (+1 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (36): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+28 more)

### Community 7 - "cli crate"
Cohesion: 0.24
Nodes (9): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, AppContext, clap, cli crate, core crate (+1 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.10
Nodes (15): connect(), MINIMAL_SCHEMA, ws_connection_address(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_and_pull_preserves_relation_edge(), cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql() (+7 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.06
Nodes (27): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+19 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.08
Nodes (42): Model, NamingConvention, creates_initial_migration_from_empty_snapshot(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions() (+34 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.12
Nodes (28): NamingContext, preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table(), render_define_table_op() (+20 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.23
Nodes (6): DatabaseSchema, merge_pulled_schema(), SchemaIntrospector, DbPullInput, DbPullOutput, DbPullUseCase

### Community 16 - "DomainError"
Cohesion: 0.07
Nodes (20): FsAdapter, list_schema_files(), load_directory_schema(), SchemaFileSource, DomainError, CodegenError, DatabaseError, MigrationError (+12 more)

### Community 18 - "validation.rs"
Cohesion: 0.27
Nodes (17): accepts_relation_field_naming_existing_edge(), rejects_edge_with_unknown_endpoint_model(), rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field() (+9 more)

### Community 19 - "mapped_name"
Cohesion: 0.27
Nodes (3): map_field_path(), mapped_name(), NamingContext<'a>

### Community 20 - "DatabaseConfig"
Cohesion: 0.13
Nodes (13): introspect_async(), SurrealDbIntrospector, ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql() (+5 more)

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.18
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "FieldType"
Cohesion: 0.07
Nodes (45): maps_all_field_types_to_rust(), schema_with_field(), default_generator_is_constructible(), maps_all_field_types_to_typescript(), schema_with_field(), FieldType, Array, Bool (+37 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.26
Nodes (12): UserDatabaseRepository, UserRepository, AppContext, Cli, DomainError, FileSystemPort, ImportUsersCommand, ImportUsersInput (+4 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "Field"
Cohesion: 0.10
Nodes (23): Field, LinkStorage, Computed, Stored, OnDeleteAction, Cascade, Ignore, Reject (+15 more)

### Community 28 - "database_config.rs"
Cohesion: 0.29
Nodes (9): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value(), resolves_env_reference() (+1 more)

### Community 36 - "Printer"
Cohesion: 0.14
Nodes (5): run_push(), run(), dispatch(), run(), Printer

### Community 37 - "btreemap"
Cohesion: 0.18
Nodes (5): RelationEndpoints, ObjectTypeDefinition, ObjectTypeField, Relation, Generator

### Community 38 - "DomainError"
Cohesion: 0.18
Nodes (11): resolveExitCode, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError, resolve_exit_code, thiserror (+3 more)

### Community 39 - "naming.rs"
Cohesion: 0.20
Nodes (8): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case(), to_snake_case()

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "model.rs"
Cohesion: 0.24
Nodes (6): Index, Edge, Model, TableMode, Schemafull, Schemaless

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 50 - "Status Check 2026-10-06"
Cohesion: 0.06
Nodes (53): Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain (+45 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.30
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "ImportUsersUseCase"
Cohesion: 0.25
Nodes (8): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, OutputWriter

### Community 55 - "SchemaSource"
Cohesion: 0.15
Nodes (7): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 57 - "ImportUsersUseCase"
Cohesion: 0.50
Nodes (3): ImportUsersUseCase, mockall, UserRepository

### Community 61 - "format_schema.rs"
Cohesion: 0.52
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 63 - "MigrationStore"
Cohesion: 0.15
Nodes (9): DatabaseExecutor, MigrationStore, checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase, MigrateRollbackInput, MigrateRollbackOutput (+1 more)

### Community 65 - "domain.rs"
Cohesion: 0.13
Nodes (4): SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 66 - "migrate_status.rs"
Cohesion: 0.29
Nodes (7): MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 67 - "NamingCase"
Cohesion: 0.25
Nodes (7): NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase, naming_case_str()

### Community 69 - "surrealdb_executor.rs"
Cohesion: 0.53
Nodes (4): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor

## Knowledge Gaps
- **122 isolated node(s):** `Ignore`, `Unset`, `Cascade`, `Reject`, `Stored` (+117 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 316 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `domain.rs`, `migrate_status.rs`, `generate_code.rs`, `surrealdb_executor.rs`, `surrealdb_workflow.rs`, `DatabaseSchema`, `arc`, `migrate_dev.rs`, `DatabaseConfig`, `SchemaSource`, `database_config.rs`, `format_schema.rs`, `MigrationStore`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._
- **What connects `Ignore`, `Unset`, `Cascade` to the rest of the system?**
  _122 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MigrationOperation` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `domain.rs`, `migrate_status.rs`, `db.rs`, `Printer`, `generate_code.rs`, `DatabaseSchema`, `DomainError`, `migrate_dev.rs`, `SchemaSource`, `schema_core`, `format.rs`, `format_schema.rs`, `MigrationStore`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **Should `generate_code.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `domain.rs`, `db.rs`, `generate_code.rs`, `btreemap`, `migrations/src/lib.rs`, `model.rs`, `DomainError`, `arc`, `migrate_dev.rs`, `DatabaseConfig`, `SchemaSource`, `database_config.rs`, `MigrationStore`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Should `dispatch.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05551020408163265 - nodes in this community are weakly interconnected._