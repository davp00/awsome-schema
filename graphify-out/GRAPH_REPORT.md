# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 123 files · ~47,295 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1126 nodes · 2313 edges · 73 communities (50 shown, 23 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 112 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6388a041`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- DomainError
- naming.rs
- Rust Best Practices
- Printer
- core
- Milestone 1 Offline Schema Toolchain
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
- DatabaseSchema
- ImportUsersUseCase
- Git Commit Skill
- db.rs
- schema_core
- templates.rs
- check.sh
- install-hooks.sh
- NamingConvention
- super
- mapped_name
- arc
- cargo clippy
- Understanding Pointers
- DomainError
- cli/src/lib.rs
- Error Handling
- Comments versus Documentation
- SurrealDB-First Policy
- Split-Core Hexagonal CLI Architecture
- Coding Styles and Idioms
- core crate
- migrate_dev.rs
- MigrationStore
- SchemaSource
- FsAdapter
- domain.rs
- surrealdb_executor.rs
- Status Check 2026-10-06
- format_schema.rs
- di.rs
- NamingCase
- Client Generators
- usecases/mod.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 79 edges
2. `DatabaseSchema` - 53 edges
3. `AppContext` - 37 edges
4. `FileSystemPort` - 29 edges
5. `FieldType` - 28 edges
6. `Parser` - 27 edges
7. `MigrationOperation` - 26 edges
8. `Token` - 26 edges
9. `Printer` - 26 edges
10. `validate_schema()` - 18 edges

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

## Communities (73 total, 23 thin omitted)

### Community 0 - "DomainError"
Cohesion: 0.08
Nodes (40): Field, FieldType, Array, Bool, Custom, Datetime, Float, Int (+32 more)

### Community 1 - "naming.rs"
Cohesion: 0.18
Nodes (9): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingContext, split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case() (+1 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "Printer"
Cohesion: 0.13
Nodes (5): run(), run(), dispatch(), run(), Printer

### Community 4 - "core"
Cohesion: 0.08
Nodes (16): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, SchemaRenderer (+8 more)

### Community 5 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.15
Nodes (18): Milestone 1 Offline Schema Toolchain, Post-Plan Work on main, Roadmap Notes, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, SurrealDB Rust SDK Upgrade (+10 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (34): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+26 more)

### Community 7 - "cli crate"
Cohesion: 0.22
Nodes (10): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, anyhow, AppContext, AppContext, clap, cli crate (+2 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.08
Nodes (15): connect(), MINIMAL_SCHEMA, ws_connection_address(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_and_pull_preserves_relation_edge(), cli_db_push_applies_schema_to_surrealdb(), cli_generate_outputs_schema_sql() (+7 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.07
Nodes (23): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+15 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (24): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+16 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.09
Nodes (37): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+29 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (26): RelationEndpoints, preserves_field_name_when_fields_naming_not_set(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table(), render_define_table_op(), render_edge_schema() (+18 more)

### Community 15 - "AppContext"
Cohesion: 0.15
Nodes (5): run_apply(), run_create(), run_dev(), run_status(), AppContext

### Community 16 - "FileSystemPort"
Cohesion: 0.18
Nodes (4): list_schema_files(), load_directory_schema(), SchemaFileSource, FileSystemPort

### Community 17 - "src/print.rs"
Cohesion: 0.13
Nodes (16): field_type_for_print(), naming_case_str(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field(), print_field_type() (+8 more)

### Community 18 - "validation.rs"
Cohesion: 0.28
Nodes (16): accepts_relation_field_naming_existing_edge(), rejects_edge_with_unknown_endpoint_model(), rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field() (+8 more)

### Community 19 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 20 - "database_config.rs"
Cohesion: 0.22
Nodes (10): builds_config_from_literal_url(), DatabaseConfig, normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+2 more)

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.18
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.25
Nodes (5): cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "DatabaseSchema"
Cohesion: 0.06
Nodes (39): introspect_async(), SurrealDbIntrospector, StubIntrospector, default_generator_is_constructible(), maps_all_field_types_to_typescript(), schema_with_field(), DatabaseSchema, Generator (+31 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.10
Nodes (25): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, resolveExitCode, User (+17 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "db.rs"
Cohesion: 0.23
Nodes (10): ensure_can_write(), run_pull(), run_push(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir(), table_fragment_path() (+2 more)

### Community 36 - "NamingConvention"
Cohesion: 0.21
Nodes (7): Index, Edge, Model, TableMode, Schemafull, Schemaless, NamingConvention

### Community 37 - "super"
Cohesion: 0.19
Nodes (3): MigrationPlan, Relation, MigrationRenderer

### Community 39 - "mapped_name"
Cohesion: 0.31
Nodes (3): map_field_path(), mapped_name(), NamingContext<'a>

### Community 40 - "arc"
Cohesion: 0.24
Nodes (3): InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "DomainError"
Cohesion: 0.25
Nodes (8): Error Catalog, awesome-schema binary, Cargo Workspace Layout, DomainError, thiserror, DomainError, Exit Codes, db pull Not Implemented

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
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 50 - "core crate"
Cohesion: 0.27
Nodes (10): Later Backlog, cli crate, Connectivity Stubs, core crate, MigrationStore, Planned Database Providers, renderers crate, SchemaIntrospector (+2 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.39
Nodes (4): MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "MigrationStore"
Cohesion: 0.17
Nodes (5): MigrationStoreAdapter, MigrationStore, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase

### Community 55 - "SchemaSource"
Cohesion: 0.25
Nodes (4): SchemaSource, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 58 - "domain.rs"
Cohesion: 0.12
Nodes (7): DatabaseExecutor, DbPushInput, DbPushOutput, DbPushUseCase, MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase

### Community 59 - "surrealdb_executor.rs"
Cohesion: 0.36
Nodes (4): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor

### Community 60 - "Status Check 2026-10-06"
Cohesion: 0.31
Nodes (10): Migration Lifecycle, db push, DropIndex Gap, migrate apply Gap, README Drift, Status Check 2026-10-06, MigrationOperation, MigrationRenderer (+2 more)

### Community 61 - "format_schema.rs"
Cohesion: 0.42
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 62 - "di.rs"
Cohesion: 0.38
Nodes (3): build_context(), build_context_with_introspector(), test_context_with_introspector()

### Community 63 - "NamingCase"
Cohesion: 0.29
Nodes (6): NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase

### Community 64 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

### Community 65 - "usecases/mod.rs"
Cohesion: 0.38
Nodes (3): MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase

## Knowledge Gaps
- **111 isolated node(s):** `CreateTable`, `DropTable`, `AlterTable`, `CreateField`, `DropField` (+106 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 306 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `usecases/mod.rs`, `core`, `super`, `arc`, `migrate_dev.rs`, `database_config.rs`, `MigrationStore`, `SchemaSource`, `domain.rs`?**
  _High betweenness centrality (0.115) - this node is a cross-community bridge._
- **What connects `CreateTable`, `DropTable`, `AlterTable` to the rest of the system?**
  _111 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DomainError` be split into smaller, more focused modules?**
  _Cohesion score 0.07894736842105263 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `DomainError`, `core`, `AppContext`, `FileSystemPort`, `src/print.rs`, `migrate_dev.rs`, `database_config.rs`, `MigrationStore`, `SchemaSource`, `db.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Should `Printer` be split into smaller, more focused modules?**
  _Cohesion score 0.12554112554112554 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `Printer`, `FileSystemPort`, `DatabaseSchema`, `db.rs`, `schema_core`, `format_schema.rs`, `di.rs`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Should `core` be split into smaller, more focused modules?**
  _Cohesion score 0.08097165991902834 - nodes in this community are weakly interconnected._