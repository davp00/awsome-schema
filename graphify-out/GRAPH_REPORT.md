# Graph Report - awsome-schema  (2026-10-06)

## Corpus Check
- 131 files · ~53,397 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1249 nodes · 2583 edges · 86 communities (55 shown, 31 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 127 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3cb6978a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Parser
- MigrationOperation
- Rust Best Practices
- db_push.rs
- core
- AppContext
- dispatch.rs
- cli crate
- common/mod.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- DatabaseSchema
- DomainError
- naming.rs
- validation.rs
- NamingConvention
- DatabaseConfig
- parser/src/lib.rs
- Performance Mindset
- cli
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- normalize.rs
- database_config.rs
- templates.rs
- check.sh
- install-hooks.sh
- domain.rs
- btreemap
- DomainError
- NamingCase
- cargo clippy
- Understanding Pointers
- Infrastructure Layer
- src/print.rs
- Error Handling
- Comments versus Documentation
- Field
- Split-Core Hexagonal CLI Architecture
- FieldType
- Status Check 2026-10-06
- migrate_dev.rs
- ImportUsersUseCase
- SchemaSource
- arc
- ImportUsersUseCase
- print_schema
- generate_code.rs
- cli/src/lib.rs
- TypeScriptGenerator
- VectorDist
- format_schema.rs
- migrate_status.rs
- surrealdb_executor.rs
- codegen-rust/tests/generator.rs
- codegen-typescript/tests/generator.rs
- OnDeleteAction

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 91 edges
2. `AppContext` - 39 edges
3. `DatabaseSchema` - 35 edges
4. `FileSystemPort` - 31 edges
5. `Parser` - 28 edges
6. `Printer` - 27 edges
7. `Token` - 26 edges
8. `FieldType` - 23 edges
9. `MigrationOperation` - 21 edges
10. `map_database_info()` - 20 edges

## Surprising Connections (you probably didn't know these)
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
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

## Communities (86 total, 31 thin omitted)

### Community 0 - "Parser"
Cohesion: 0.17
Nodes (6): link_target_from_type(), parse_index_fields(), parse_schema(), ParsedFieldAttributes, Parser, validate_field_rules()

### Community 1 - "MigrationOperation"
Cohesion: 0.12
Nodes (16): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+8 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "db_push.rs"
Cohesion: 0.31
Nodes (4): SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 4 - "core"
Cohesion: 0.24
Nodes (3): map_field_type(), RustGenerator, CodeGenerator

### Community 5 - "AppContext"
Cohesion: 0.07
Nodes (25): ensure_can_write(), run_pull(), run_push(), write_single_schema(), write_split_schema(), run(), run(), run() (+17 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.06
Nodes (36): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+28 more)

### Community 7 - "cli crate"
Cohesion: 0.24
Nodes (9): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, AppContext, clap, cli crate, core crate (+1 more)

### Community 8 - "common/mod.rs"
Cohesion: 0.06
Nodes (22): connect(), MINIMAL_SCHEMA, ws_connection_address(), connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA (+14 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.06
Nodes (27): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir(), init_project_skips_existing_schema(), MemoryFs (+19 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.16
Nodes (25): creates_initial_migration_from_empty_snapshot(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions(), detects_dropped_index(), detects_dropped_table() (+17 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (28): preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table(), render_define_table_op(), render_edge_schema() (+20 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.12
Nodes (9): introspect_async(), SurrealDbIntrospector, DatabaseSchema, merge_pulled_schema(), MigrationStore, SchemaIntrospector, DbPullInput, DbPullOutput (+1 more)

### Community 16 - "DomainError"
Cohesion: 0.07
Nodes (17): FsAdapter, MigrationStoreAdapter, load_directory_schema(), SchemaFileSource, DomainError, CodegenError, DatabaseError, MigrationError (+9 more)

### Community 17 - "naming.rs"
Cohesion: 0.18
Nodes (10): Model, map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingContext, split_identifier_parts(), to_camel_case(), to_kebab_case() (+2 more)

### Community 18 - "validation.rs"
Cohesion: 0.27
Nodes (17): accepts_relation_field_naming_existing_edge(), rejects_edge_with_unknown_endpoint_model(), rejects_edge_without_endpoints(), rejects_empty_schema(), rejects_flexible_on_non_object_field(), rejects_missing_provider(), rejects_model_without_fields(), rejects_model_without_id_field() (+9 more)

### Community 19 - "NamingConvention"
Cohesion: 0.22
Nodes (6): Edge, Model, TableMode, Schemafull, Schemaless, NamingConvention

### Community 20 - "DatabaseConfig"
Cohesion: 0.14
Nodes (12): ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql(), SurrealDbMigrationLedger, with_runtime() (+4 more)

### Community 21 - "parser/src/lib.rs"
Cohesion: 0.18
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.39
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "introspect.rs"
Cohesion: 0.13
Nodes (32): Index, ComputedBacklink, edge_name_for_table(), is_relation_table(), map_database_info(), map_fields(), map_indexes(), map_record_links() (+24 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.26
Nodes (12): UserDatabaseRepository, UserRepository, AppContext, Cli, DomainError, FileSystemPort, ImportUsersCommand, ImportUsersInput (+4 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.33
Nodes (12): ensure_on_delete_default(), expands_object_type_reference_into_nested_fields(), infer_anonymous_pair_names(), LinkRef, normalize_model_object_types(), normalize_schema(), resolve_link_pairs(), resolve_stored_computed() (+4 more)

### Community 28 - "database_config.rs"
Cohesion: 0.29
Nodes (9): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value(), resolves_env_reference() (+1 more)

### Community 36 - "domain.rs"
Cohesion: 0.11
Nodes (8): DatabaseExecutor, checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase, MigrateRollbackInput, MigrateRollbackOutput, MigrateRollbackUseCase

### Community 37 - "btreemap"
Cohesion: 0.14
Nodes (3): RelationEndpoints, Relation, Generator

### Community 38 - "DomainError"
Cohesion: 0.20
Nodes (10): resolveExitCode, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError, resolve_exit_code, thiserror (+2 more)

### Community 40 - "NamingCase"
Cohesion: 0.14
Nodes (9): map_field_path(), mapped_name(), NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase (+1 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 44 - "src/print.rs"
Cohesion: 0.20
Nodes (9): field_type_for_print(), naming_case_str(), print_edge(), print_edge_block(), print_field(), print_field_type(), print_model(), print_model_block() (+1 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "Field"
Cohesion: 0.20
Nodes (6): Field, LinkStorage, Computed, Stored, nested_fields_from_object_body(), nested_fields_from_object_type()

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 49 - "FieldType"
Cohesion: 0.15
Nodes (12): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+4 more)

### Community 50 - "Status Check 2026-10-06"
Cohesion: 0.06
Nodes (54): Client Generators, db pull, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain (+46 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.30
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "ImportUsersUseCase"
Cohesion: 0.25
Nodes (8): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, OutputWriter

### Community 55 - "SchemaSource"
Cohesion: 0.15
Nodes (7): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "arc"
Cohesion: 0.13
Nodes (5): list_schema_files(), StubIntrospector, InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 57 - "ImportUsersUseCase"
Cohesion: 0.50
Nodes (3): ImportUsersUseCase, mockall, UserRepository

### Community 60 - "print_schema"
Cohesion: 0.19
Nodes (8): print_config_blocks(), print_datasource(), print_generator(), print_naming(), print_object_type(), print_object_type_field(), print_schema(), prints_index_kinds_round_trip()

### Community 61 - "generate_code.rs"
Cohesion: 0.27
Nodes (8): CodeGeneratorPort, GenerateCodeInput, GenerateCodeOutput, GenerateCodeTarget, Rust, Schema, TypeScript, GenerateCodeUseCase

### Community 64 - "VectorDist"
Cohesion: 0.29
Nodes (4): VectorDist, Cosine, Euclidean, Manhattan

### Community 65 - "format_schema.rs"
Cohesion: 0.52
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 66 - "migrate_status.rs"
Cohesion: 0.36
Nodes (6): MigrateStatusInput, MigrateStatusOutput, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 67 - "surrealdb_executor.rs"
Cohesion: 0.53
Nodes (4): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor

### Community 69 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.47
Nodes (3): default_generator_is_constructible(), maps_all_field_types_to_typescript(), schema_with_field()

### Community 72 - "OnDeleteAction"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

## Knowledge Gaps
- **127 isolated node(s):** `Euclidean`, `Cosine`, `Manhattan`, `LeftBrace`, `RightBrace` (+122 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 348 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **31 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `format_schema.rs`, `migrate_status.rs`, `surrealdb_executor.rs`, `core`, `domain.rs`, `db_push.rs`, `common/mod.rs`, `DatabaseSchema`, `migrate_dev.rs`, `DatabaseConfig`, `SchemaSource`, `arc`, `database_config.rs`, `generate_code.rs`, `TypeScriptGenerator`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **What connects `Euclidean`, `Cosine`, `Manhattan` to the rest of the system?**
  _127 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MigrationOperation` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `format_schema.rs`, `db_push.rs`, `domain.rs`, `DatabaseSchema`, `DomainError`, `migrate_dev.rs`, `DatabaseConfig`, `SchemaSource`, `arc`, `generate_code.rs`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Should `AppContext` be split into smaller, more focused modules?**
  _Cohesion score 0.07337526205450734 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `db_push.rs`, `core`, `btreemap`, `AppContext`, `DomainError`, `FieldType`, `NamingConvention`, `migrate_dev.rs`, `SchemaSource`, `database_config.rs`, `generate_code.rs`, `TypeScriptGenerator`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Should `dispatch.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05551020408163265 - nodes in this community are weakly interconnected._