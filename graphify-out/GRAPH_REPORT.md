# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- Corpus is ~42,015 words - fits in a single context window. You may not need a graph.

## Summary
- 937 nodes · 1978 edges · 36 communities (27 shown, 9 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 98 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Schema Parser Core
- Naming and Config
- Rust Best Practices
- CLI App Context
- Code Generation Pipeline
- Roadmap and Product
- CLI Command Dispatch
- Hexagonal CLI Skill
- SurrealDB E2E Tests
- Testing and Docs
- Core Use Case Tests
- Schema Lexer
- Schema Diff Engine
- SurrealDB SQL Renderer
- Migration Store
- Init Project Filesystem
- Migrate Dev Planning
- Schema Validation Rules
- Hexagonal Layer Rules
- Migrate Apply Execution
- Validate Schema Use Case
- Db Push Use Case
- Cargo Workspace Crates
- Migrate Status Use Case
- Git Commit Skill
- Format Schema Use Case
- Migrate Create Use Case
- Default Schema Template
- Workspace Check Script
- Hook Install Script

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
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `awesome-schema binary` --conceptually_related_to--> `cli crate`  [INFERRED]
  .agents/skills/split-core-hexagonal-cli/references/RUST.md → README.md
- `id Field Enforcement` --conceptually_related_to--> `Awesome Schema DSL`  [INFERRED]
  docs/roadmap/surrealdb-3.3.md → README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Rust Test Layers** — agents_skills_rust_best_practices_references_chapter_05_unit_tests, agents_skills_rust_best_practices_references_chapter_05_integration_tests, agents_skills_rust_best_practices_references_chapter_05_doc_tests [EXTRACTED 1.00]
- **Static and Dynamic Dispatch Trade-off** — agents_skills_rust_best_practices_references_chapter_06_generics, agents_skills_rust_best_practices_references_chapter_06_static_dispatch, agents_skills_rust_best_practices_references_chapter_06_dynamic_dispatch [EXTRACTED 1.00]
- **Library and Binary Error Strategy** — agents_skills_rust_best_practices_references_chapter_04_result_over_panic, agents_skills_rust_best_practices_references_chapter_04_thiserror, agents_skills_rust_best_practices_references_chapter_04_anyhow, agents_skills_rust_best_practices_references_chapter_04_question_mark [EXTRACTED 1.00]
- **Split-Core Architectural Layers** — agents_skills_split_core_hexagonal_cli_references_reference_domain_core, agents_skills_split_core_hexagonal_cli_references_reference_application_core, agents_skills_split_core_hexagonal_cli_references_reference_infrastructure_layer, agents_skills_split_core_hexagonal_cli_references_reference_shared_capability [EXTRACTED 1.00]
- **Awesome Schema Provider Ports** — readme_schema_renderer, readme_migration_renderer, readme_schema_introspector, readme_migration_store, readme_schema_source [EXTRACTED 1.00]
- **Faithful SurrealDB Schema Loop** — docs_roadmap_next_surrealdb_first, docs_roadmap_next_graph_edges, docs_roadmap_next_migration_lifecycle, docs_roadmap_next_db_pull, docs_roadmap_next_index_kinds [EXTRACTED 1.00]

## Communities (36 total, 9 thin omitted)

### Community 0 - "Schema Parser Core"
Cohesion: 0.07
Nodes (41): FsAdapter, Field, FieldType, Array, Bool, Custom, Datetime, Float (+33 more)

### Community 1 - "Naming and Config"
Cohesion: 0.05
Nodes (35): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value(), resolves_env_reference() (+27 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.06
Nodes (29): Coding Styles and Idioms, rustfmt, cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect (+21 more)

### Community 3 - "CLI App Context"
Cohesion: 0.06
Nodes (22): SurrealDbIntrospector, run_pull(), run_push(), run(), run(), run(), run_apply(), run_create() (+14 more)

### Community 4 - "Code Generation Pipeline"
Cohesion: 0.05
Nodes (33): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+25 more)

### Community 5 - "Roadmap and Product"
Cohesion: 0.06
Nodes (55): DomainError, Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer (+47 more)

### Community 6 - "CLI Command Dispatch"
Cohesion: 0.05
Nodes (32): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+24 more)

### Community 7 - "Hexagonal CLI Skill"
Cohesion: 0.06
Nodes (42): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, CLI Architecture Checklist, CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase (+34 more)

### Community 8 - "SurrealDB E2E Tests"
Cohesion: 0.08
Nodes (15): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, MINIMAL_SCHEMA, ws_connection_address(), cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql() (+7 more)

### Community 10 - "Testing and Docs"
Cohesion: 0.09
Nodes (17): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Architectural Decision Record (+9 more)

### Community 11 - "Core Use Case Tests"
Cohesion: 0.11
Nodes (17): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+9 more)

### Community 12 - "Schema Lexer"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "Schema Diff Engine"
Cohesion: 0.11
Nodes (28): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+20 more)

### Community 14 - "SurrealDB SQL Renderer"
Cohesion: 0.20
Nodes (23): NamingContext, NamingConvention, preserves_field_name_when_fields_naming_not_set(), render_define_field(), render_define_index(), render_edge_schema(), render_model_schema(), render_operation() (+15 more)

### Community 15 - "Migration Store"
Cohesion: 0.14
Nodes (5): MigrationStoreAdapter, DatabaseSchema, MigrationStore, MemoryMigrationStore, StaticSchemaSource

### Community 16 - "Init Project Filesystem"
Cohesion: 0.15
Nodes (5): SchemaFileSource, FileSystemPort, InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 17 - "Migrate Dev Planning"
Cohesion: 0.20
Nodes (8): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort, RecordingRenderer, StaticDiff

### Community 18 - "Schema Validation Rules"
Cohesion: 0.38
Nodes (13): rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field(), rejects_nested_field_without_parent_object(), rejects_wrong_id_field_type() (+5 more)

### Community 19 - "Hexagonal Layer Rules"
Cohesion: 0.21
Nodes (6): resolveExitCode, AppLogger, Error Catalog, Orchestrator, Shared Capability Package, Exit Codes

### Community 20 - "Migrate Apply Execution"
Cohesion: 0.22
Nodes (6): DatabaseConfig, DatabaseExecutor, MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase, RecordingDatabase

### Community 21 - "Validate Schema Use Case"
Cohesion: 0.29
Nodes (4): SchemaSource, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 22 - "Db Push Use Case"
Cohesion: 0.31
Nodes (4): SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 23 - "Cargo Workspace Crates"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 25 - "Migrate Status Use Case"
Cohesion: 0.32
Nodes (3): MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "Format Schema Use Case"
Cohesion: 0.48
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 28 - "Migrate Create Use Case"
Cohesion: 0.38
Nodes (3): MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase

## Knowledge Gaps
- **107 isolated node(s):** `Init`, `Validate`, `Format`, `Generate`, `Migrate` (+102 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 246 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **9 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `Schema Parser Core` to `Naming and Config`, `CLI App Context`, `Code Generation Pipeline`, `SurrealDB E2E Tests`, `Core Use Case Tests`, `SurrealDB SQL Renderer`, `Migration Store`, `Init Project Filesystem`, `Migrate Dev Planning`, `Schema Validation Rules`, `Migrate Apply Execution`, `Validate Schema Use Case`, `Db Push Use Case`, `Migrate Status Use Case`, `Format Schema Use Case`, `Migrate Create Use Case`?**
  _High betweenness centrality (0.174) - this node is a cross-community bridge._
- **What connects `Init`, `Validate`, `Format` to the rest of the system?**
  _107 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Schema Parser Core` be split into smaller, more focused modules?**
  _Cohesion score 0.07023214810461358 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `CLI App Context` to `Code Generation Pipeline`, `Migration Store`, `Init Project Filesystem`, `Migrate Dev Planning`, `Migrate Apply Execution`, `Validate Schema Use Case`, `Db Push Use Case`, `Migrate Status Use Case`, `Format Schema Use Case`, `Migrate Create Use Case`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._
- **Should `Naming and Config` be split into smaller, more focused modules?**
  _Cohesion score 0.05046948356807512 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `Migration Store` to `Schema Parser Core`, `Naming and Config`, `CLI App Context`, `Code Generation Pipeline`, `Core Use Case Tests`, `Schema Diff Engine`, `SurrealDB SQL Renderer`, `Init Project Filesystem`, `Migrate Dev Planning`, `Schema Validation Rules`, `Validate Schema Use Case`, `Db Push Use Case`?**
  _High betweenness centrality (0.061) - this node is a cross-community bridge._
- **Should `Rust Best Practices` be split into smaller, more focused modules?**
  _Cohesion score 0.061457418788410885 - nodes in this community are weakly interconnected._