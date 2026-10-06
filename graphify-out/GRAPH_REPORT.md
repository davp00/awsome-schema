# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 123 files · ~46,022 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1089 nodes · 2225 edges · 55 communities (39 shown, 16 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 111 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0ec6b2bb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- DomainError
- naming.rs
- Rust Best Practices
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
- DatabaseSchema
- ImportUsersUseCase
- Git Commit Skill
- db.rs
- format.rs
- templates.rs
- check.sh
- install-hooks.sh
- MigrationOperation
- btreemap
- mapped_name
- NamingContext<'a>
- NamingCase
- cargo clippy
- Understanding Pointers
- DomainError
- cli/src/lib.rs
- Error Handling
- Comments versus Documentation
- ImportUsersUseCase
- Split-Core Hexagonal CLI Architecture
- Coding Styles and Idioms
- ImportUsersUseCase
- validate.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 81 edges
2. `DatabaseSchema` - 70 edges
3. `AppContext` - 37 edges
4. `FileSystemPort` - 30 edges
5. `FieldType` - 28 edges
6. `Parser` - 27 edges
7. `Printer` - 26 edges
8. `Token` - 26 edges
9. `MigrationOperation` - 22 edges
10. `Rust Best Practices` - 18 edges

## Surprising Connections (you probably didn't know these)
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `awesome-schema binary` --conceptually_related_to--> `cli crate`  [INFERRED]
  .agents/skills/split-core-hexagonal-cli/references/RUST.md → README.md
- `db pull Not Implemented` --conceptually_related_to--> `DomainError`  [INFERRED]
  docs/roadmap/status.md → .agents/skills/split-core-hexagonal-cli/SKILL.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Awesome Schema Provider Ports** — readme_schema_renderer, readme_migration_renderer, readme_schema_introspector, readme_migration_store, readme_schema_source [EXTRACTED 1.00]
- **Faithful SurrealDB Schema Loop** — docs_roadmap_next_surrealdb_first, docs_roadmap_next_graph_edges, docs_roadmap_next_migration_lifecycle, docs_roadmap_next_db_pull, docs_roadmap_next_index_kinds [EXTRACTED 1.00]
- **Library and Binary Error Strategy** — agents_skills_rust_best_practices_references_chapter_04_result_over_panic, agents_skills_rust_best_practices_references_chapter_04_thiserror, agents_skills_rust_best_practices_references_chapter_04_anyhow, agents_skills_rust_best_practices_references_chapter_04_question_mark [EXTRACTED 1.00]
- **Rust Test Layers** — agents_skills_rust_best_practices_references_chapter_05_unit_tests, agents_skills_rust_best_practices_references_chapter_05_integration_tests, agents_skills_rust_best_practices_references_chapter_05_doc_tests [EXTRACTED 1.00]
- **Split-Core Architectural Layers** — agents_skills_split_core_hexagonal_cli_references_reference_domain_core, agents_skills_split_core_hexagonal_cli_references_reference_application_core, agents_skills_split_core_hexagonal_cli_references_reference_infrastructure_layer, agents_skills_split_core_hexagonal_cli_references_reference_shared_capability [EXTRACTED 1.00]
- **Static and Dynamic Dispatch Trade-off** — agents_skills_rust_best_practices_references_chapter_06_generics, agents_skills_rust_best_practices_references_chapter_06_static_dispatch, agents_skills_rust_best_practices_references_chapter_06_dynamic_dispatch [EXTRACTED 1.00]

## Communities (55 total, 16 thin omitted)

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
Cohesion: 0.16
Nodes (7): run_push(), run_apply(), run_create(), run_dev(), run_status(), dispatch(), Printer

### Community 4 - "core"
Cohesion: 0.07
Nodes (19): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+11 more)

### Community 5 - "Status Check 2026-10-06"
Cohesion: 0.06
Nodes (54): Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain (+46 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (34): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+26 more)

### Community 7 - "cli crate"
Cohesion: 0.24
Nodes (9): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, AppContext, clap, cli crate, core crate (+1 more)

### Community 8 - "surrealdb_workflow.rs"
Cohesion: 0.06
Nodes (17): connect(), execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, MINIMAL_SCHEMA, ws_connection_address(), cli_db_pull_overwrites_schema_after_push() (+9 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.08
Nodes (21): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+13 more)

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
Cohesion: 0.14
Nodes (4): AppContext, build_context(), build_context_with_introspector(), test_context_with_introspector()

### Community 16 - "FileSystemPort"
Cohesion: 0.07
Nodes (13): FsAdapter, MigrationStoreAdapter, list_schema_files(), load_directory_schema(), SchemaFileSource, FileSystemPort, FormatSchemaInput, FormatSchemaOutput (+5 more)

### Community 17 - "src/print.rs"
Cohesion: 0.13
Nodes (17): Generator, field_type_for_print(), naming_case_str(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field() (+9 more)

### Community 18 - "validation.rs"
Cohesion: 0.07
Nodes (30): MigrationPlan, MigrationRenderer, MigrationStore, SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, MigrateDevInput (+22 more)

### Community 19 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 20 - "database_config.rs"
Cohesion: 0.08
Nodes (18): builds_config_from_literal_url(), DatabaseConfig, normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+10 more)

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.19
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.25
Nodes (5): cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.44
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "DatabaseSchema"
Cohesion: 0.05
Nodes (35): introspect_async(), SurrealDbIntrospector, StubIntrospector, DatabaseSchema, merge_pulled_schema(), SchemaIntrospector, DbPullInput, DbPullOutput (+27 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.26
Nodes (12): UserDatabaseRepository, UserRepository, AppContext, Cli, DomainError, FileSystemPort, ImportUsersCommand, ImportUsersInput (+4 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "db.rs"
Cohesion: 0.26
Nodes (9): ensure_can_write(), run_pull(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir(), table_fragment_path(), TABLE_SUFFIX (+1 more)

### Community 28 - "format.rs"
Cohesion: 0.17
Nodes (3): run(), run(), run()

### Community 36 - "MigrationOperation"
Cohesion: 0.13
Nodes (15): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+7 more)

### Community 38 - "mapped_name"
Cohesion: 0.23
Nodes (7): Index, Edge, Model, TableMode, Schemafull, Schemaless, mapped_name()

### Community 40 - "NamingCase"
Cohesion: 0.29
Nodes (6): NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "DomainError"
Cohesion: 0.20
Nodes (10): resolveExitCode, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError, resolve_exit_code, thiserror (+2 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "ImportUsersUseCase"
Cohesion: 0.25
Nodes (8): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, OutputWriter

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 50 - "ImportUsersUseCase"
Cohesion: 0.50
Nodes (3): ImportUsersUseCase, mockall, UserRepository

## Knowledge Gaps
- **111 isolated node(s):** `Init`, `Validate`, `Format`, `Generate`, `Migrate` (+106 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 301 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DatabaseSchema` connect `DatabaseSchema` to `DomainError`, `core`, `btreemap`, `usecases.rs`, `migrations/src/lib.rs`, `surrealdb/mod.rs`, `AppContext`, `FileSystemPort`, `src/print.rs`, `validation.rs`, `database_config.rs`, `parser/src/lib.rs`, `db.rs`?**
  _High betweenness centrality (0.187) - this node is a cross-community bridge._
- **What connects `Init`, `Validate`, `Format` to the rest of the system?**
  _111 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DomainError` be split into smaller, more focused modules?**
  _Cohesion score 0.07894736842105263 - nodes in this community are weakly interconnected._
- **Why does `DomainError` connect `DomainError` to `FileSystemPort`, `validation.rs`, `core`, `database_config.rs`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Should `core` be split into smaller, more focused modules?**
  _Cohesion score 0.0708245243128964 - nodes in this community are weakly interconnected._
- **Why does `Token` connect `Token` to `DomainError`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **Should `Status Check 2026-10-06` be split into smaller, more focused modules?**
  _Cohesion score 0.05844155844155844 - nodes in this community are weakly interconnected._