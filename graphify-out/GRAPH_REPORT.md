# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 116 files · ~42,061 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 939 nodes · 1980 edges · 42 communities (31 shown, 11 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 98 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a0dce683`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- DomainError
- naming.rs
- Rust Best Practices
- AppContext
- parser/src/lib.rs
- Status Check 2026-10-06
- dispatch.rs
- ImportUsersUseCase
- surrealdb_workflow.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- DatabaseSchema
- FileSystemPort
- MigrationPlan
- validation.rs
- Infrastructure Layer
- database_config.rs
- SchemaSource
- usecases/mod.rs
- cli
- MigrationStore
- Git Commit Skill
- format_schema.rs
- migrate_create.rs
- templates.rs
- check.sh
- install-hooks.sh
- MigrationOperation
- btreemap
- NamingConvention
- mapped_name
- NamingCase

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 117 edges
2. `DatabaseSchema` - 54 edges
3. `AppContext` - 33 edges
4. `FieldType` - 28 edges
5. `Parser` - 27 edges
6. `Token` - 26 edges
7. `Printer` - 25 edges
8. `FileSystemPort` - 24 edges
9. `MigrationOperation` - 23 edges
10. `SchemaSource` - 19 edges

## Surprising Connections (you probably didn't know these)
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
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

## Communities (42 total, 11 thin omitted)

### Community 0 - "DomainError"
Cohesion: 0.07
Nodes (41): FsAdapter, Field, FieldType, Array, Bool, Custom, Datetime, Float (+33 more)

### Community 1 - "naming.rs"
Cohesion: 0.20
Nodes (8): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case(), to_snake_case()

### Community 2 - "Rust Best Practices"
Cohesion: 0.06
Nodes (29): Coding Styles and Idioms, rustfmt, cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect (+21 more)

### Community 3 - "AppContext"
Cohesion: 0.06
Nodes (22): SurrealDbIntrospector, run_pull(), run_push(), run(), run(), run(), run_apply(), run_create() (+14 more)

### Community 4 - "parser/src/lib.rs"
Cohesion: 0.05
Nodes (33): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+25 more)

### Community 5 - "Status Check 2026-10-06"
Cohesion: 0.06
Nodes (53): Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain (+45 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (32): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+24 more)

### Community 7 - "ImportUsersUseCase"
Cohesion: 0.05
Nodes (47): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, CLI Architecture Checklist, CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase (+39 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.11
Nodes (11): MINIMAL_SCHEMA, cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql(), cli_init_creates_project_files(), cli_init_validate_and_migrate_against_surrealdb(), db_pull_is_not_implemented_yet(), docker_unavailable(), MINIMAL_SCHEMA (+3 more)

### Community 10 - "Automated Testing"
Cohesion: 0.09
Nodes (17): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Architectural Decision Record (+9 more)

### Community 11 - "usecases.rs"
Cohesion: 0.11
Nodes (17): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+9 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.27
Nodes (13): creates_initial_migration_from_empty_snapshot(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_dropped_table(), diff_model(), diff_schemas(), down_migration_reverses_up_changes() (+5 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.22
Nodes (22): NamingContext, preserves_field_name_when_fields_naming_not_set(), render_define_field(), render_define_index(), render_edge_schema(), render_model_schema(), render_operation(), render_unique_index() (+14 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.17
Nodes (4): MigrationStoreAdapter, DatabaseSchema, MemoryMigrationStore, StaticSchemaSource

### Community 16 - "FileSystemPort"
Cohesion: 0.15
Nodes (5): SchemaFileSource, FileSystemPort, InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 17 - "MigrationPlan"
Cohesion: 0.18
Nodes (8): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort, RecordingRenderer, StaticDiff

### Community 18 - "validation.rs"
Cohesion: 0.38
Nodes (13): rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field(), rejects_nested_field_without_parent_object(), rejects_wrong_id_field_type() (+5 more)

### Community 19 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 20 - "database_config.rs"
Cohesion: 0.09
Nodes (20): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, builds_config_from_literal_url(), DatabaseConfig, normalize_ws_endpoint(), normalizes_http_url() (+12 more)

### Community 21 - "SchemaSource"
Cohesion: 0.29
Nodes (4): SchemaSource, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 22 - "usecases/mod.rs"
Cohesion: 0.24
Nodes (4): SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 25 - "MigrationStore"
Cohesion: 0.25
Nodes (4): MigrationStore, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "format_schema.rs"
Cohesion: 0.48
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 28 - "migrate_create.rs"
Cohesion: 0.47
Nodes (3): MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase

### Community 36 - "MigrationOperation"
Cohesion: 0.13
Nodes (15): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+7 more)

### Community 38 - "NamingConvention"
Cohesion: 0.21
Nodes (8): Index, Edge, Model, TableMode, Schemafull, Schemaless, NamingConvention, render_define_table()

### Community 39 - "mapped_name"
Cohesion: 0.24
Nodes (3): map_field_path(), mapped_name(), NamingContext<'a>

### Community 40 - "NamingCase"
Cohesion: 0.29
Nodes (6): NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase

## Knowledge Gaps
- **107 isolated node(s):** `MINIMAL_SCHEMA`, `Relation`, `Lexer`, `Array`, `Bool` (+102 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 247 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `AppContext`, `parser/src/lib.rs`, `usecases.rs`, `surrealdb/mod.rs`, `DatabaseSchema`, `FileSystemPort`, `MigrationPlan`, `validation.rs`, `database_config.rs`, `SchemaSource`, `usecases/mod.rs`, `MigrationStore`, `format_schema.rs`, `migrate_create.rs`?**
  _High betweenness centrality (0.173) - this node is a cross-community bridge._
- **What connects `MINIMAL_SCHEMA`, `Relation`, `Lexer` to the rest of the system?**
  _107 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DomainError` be split into smaller, more focused modules?**
  _Cohesion score 0.07023214810461358 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `parser/src/lib.rs`, `DatabaseSchema`, `FileSystemPort`, `MigrationPlan`, `database_config.rs`, `SchemaSource`, `usecases/mod.rs`, `MigrationStore`, `format_schema.rs`, `migrate_create.rs`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._
- **Should `Rust Best Practices` be split into smaller, more focused modules?**
  _Cohesion score 0.061457418788410885 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `DomainError`, `AppContext`, `parser/src/lib.rs`, `btreemap`, `NamingConvention`, `usecases.rs`, `migrations/src/lib.rs`, `surrealdb/mod.rs`, `FileSystemPort`, `MigrationPlan`, `validation.rs`, `database_config.rs`, `SchemaSource`, `usecases/mod.rs`, `MigrationStore`?**
  _High betweenness centrality (0.061) - this node is a cross-community bridge._
- **Should `AppContext` be split into smaller, more focused modules?**
  _Cohesion score 0.0597567424643046 - nodes in this community are weakly interconnected._