# Graph Report - awsome-schema  (2026-10-07)

## Corpus Check
- 134 files · ~62,808 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 4, .xml 3, .surql 2)

## Summary
- 1434 nodes · 3057 edges · 114 communities (54 shown, 60 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d62ebdf7`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FieldType
- MigrationOperation
- Rust Best Practices
- Commands
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
- FsAdapter
- coverage_gaps.rs
- validation.rs
- MigrationStore
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
- arc
- btreemap
- Status Check 2026-10-06
- Post-Plan Work on main
- cargo clippy
- Understanding Pointers
- domain.rs
- src/print.rs
- Error Handling
- Comments versus Documentation
- field.rs
- Split-Core Hexagonal CLI Architecture
- offline_cli.rs
- core crate
- migrate_dev.rs
- Milestone 1 Offline Schema Toolchain
- SchemaSource
- DomainError
- Client Generators
- manual-test-indexes.sh
- generate_code.rs
- ports/mod.rs
- schema_core
- VectorDist
- Infrastructure Layer
- MigrationLedger
- require_surrealdb
- MigrationStoreAdapter
- di.rs
- TestProject
- commands.rs
- format_schema.rs
- StubIntrospector
- object_type.rs
- coverage.sh

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 76 edges
2. `map_database_info()` - 41 edges
3. `FieldType` - 38 edges
4. `Model` - 37 edges
5. `DatabaseConfig` - 36 edges
6. `validate_schema()` - 36 edges
7. `sample_schema()` - 33 edges
8. `AppContext` - 33 edges
9. `Field` - 32 edges
10. `DatabaseSchema` - 29 edges

## Surprising Connections (you probably didn't know these)
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `db pull Not Implemented` --conceptually_related_to--> `DomainError`  [INFERRED]
  docs/roadmap/status.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
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

## Communities (114 total, 60 thin omitted)

### Community 0 - "FieldType"
Cohesion: 0.13
Nodes (16): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+8 more)

### Community 1 - "MigrationOperation"
Cohesion: 0.12
Nodes (16): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+8 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "Commands"
Cohesion: 0.09
Nodes (22): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+14 more)

### Community 4 - "core"
Cohesion: 0.09
Nodes (11): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, map_field_type(), TypeScriptGenerator, default_generator_is_constructible() (+3 more)

### Community 5 - "AppContext"
Cohesion: 0.13
Nodes (11): run(), run(), run(), run_apply(), run_create(), run_dev(), run_rollback(), run_status() (+3 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.10
Nodes (16): dispatch_db_pull_refuses_overwrite_without_force(), dispatch_db_pull_reports_connection_error_without_server(), dispatch_db_push_returns_connection_error_without_server(), dispatch_format_write_command(), dispatch_generate_targets(), dispatch_init_command(), dispatch_init_when_schema_already_exists(), dispatch_migrate_apply_and_rollback_arms() (+8 more)

### Community 7 - "cli crate"
Cohesion: 0.22
Nodes (10): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, anyhow, AppContext, AppContext, clap, cli crate (+2 more)

### Community 8 - "common/mod.rs"
Cohesion: 0.16
Nodes (7): connect(), ws_connection_address(), connect(), field_define(), MINIMAL_SCHEMA, RECORD_REF_SCHEMA_TEMPLATE, table_exists()

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.07
Nodes (33): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_multiple_files_without_write_back(), format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), format_schema_writes_back_multiple_files(), generate_code_targets_schema_rust_and_typescript(), init_project_creates_schema_and_migrations_dir() (+25 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.06
Nodes (56): Edge, Model, TableMode, Schemafull, Schemaless, map_attribute_overrides_convention(), map_field_path(), mapped_name() (+48 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (30): NamingContext, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table() (+22 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.19
Nodes (7): introspect_async(), SurrealDbIntrospector, DatabaseSchema, SchemaIntrospector, DbPullInput, DbPullOutput, DbPullUseCase

### Community 17 - "coverage_gaps.rs"
Cohesion: 0.18
Nodes (10): test_context_with_introspector(), init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, run_pull_warns_when_lossy(), run_push_succeeds_with_stub_executor() (+2 more)

### Community 18 - "validation.rs"
Cohesion: 0.16
Nodes (36): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), rejects_computed_link_with_on_delete() (+28 more)

### Community 19 - "MigrationStore"
Cohesion: 0.24
Nodes (4): MigrationStore, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase

### Community 20 - "DatabaseConfig"
Cohesion: 0.10
Nodes (16): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE (+8 more)

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
Cohesion: 0.08
Nodes (60): Index, computed_without_backlink_and_weird_body_are_not_backlinks(), ComputedBacklink, edge_name_for_table(), ensure_model_id_field(), field_define_errors_propagate_through_map_fields(), id_field_non_record_keeps_scalar_type(), index_analyzer_and_hnsw_empty_values_are_ignored() (+52 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.11
Nodes (23): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, UserDatabaseRepository (+15 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.13
Nodes (41): Field, Model, LinkStorage, Computed, Stored, bare_field(), empty_base_schema(), ensure_on_delete_default() (+33 more)

### Community 28 - "database_config.rs"
Cohesion: 0.26
Nodes (9): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), preserves_wss_endpoint(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+1 more)

### Community 36 - "arc"
Cohesion: 0.18
Nodes (9): Datasource, DatabaseExecutor, checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase, MigrateRollbackInput, MigrateRollbackOutput (+1 more)

### Community 37 - "btreemap"
Cohesion: 0.15
Nodes (4): RelationEndpoints, Relation, Generator, merge_pulled_schema()

### Community 38 - "Status Check 2026-10-06"
Cohesion: 0.14
Nodes (20): db pull, Real Graph Edges, Index Kinds, Migration Lifecycle, SchemaDiffer, db pull Not Implemented, db push, DropIndex Gap (+12 more)

### Community 40 - "Post-Plan Work on main"
Cohesion: 0.29
Nodes (10): Post-Plan Work on main, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, Awesome Schema DSL, Field Assignments, Flexible Object (+2 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "domain.rs"
Cohesion: 0.13
Nodes (4): SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 44 - "src/print.rs"
Cohesion: 0.06
Nodes (38): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), NoopPull, run_pull(), run_push(), write_single_schema(), write_split_schema() (+30 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "field.rs"
Cohesion: 0.15
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.33
Nodes (7): CLI Architecture Checklist, CLI Architecture Reference, DomainError, Rust CLI Examples, Rust Implementation Guide, thiserror, DomainError

### Community 49 - "offline_cli.rs"
Cohesion: 0.17
Nodes (5): edge_schema(), record_ref_schema(), cli_db_push_and_pull_preserves_relation_edge(), cli_generate_emits_reference_and_computed_link(), cli_db_push_and_pull_preserves_record_references()

### Community 50 - "core crate"
Cohesion: 0.25
Nodes (10): awesome-schema binary, Cargo Workspace Layout, cli crate, core crate, MigrationRenderer, MigrationStore, renderers crate, SchemaIntrospector (+2 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.27
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.22
Nodes (11): Later Backlog, Milestone 1 Offline Schema Toolchain, Roadmap Notes, SurrealDB Rust SDK Upgrade, SurrealDB 3.3.0, Awesome Schema, Connectivity Stubs, Prisma (+3 more)

### Community 55 - "SchemaSource"
Cohesion: 0.29
Nodes (4): SchemaSource, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "DomainError"
Cohesion: 0.09
Nodes (16): DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError, RenderError, SchemaNotFound (+8 more)

### Community 57 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "generate_code.rs"
Cohesion: 0.27
Nodes (8): CodeGeneratorPort, GenerateCodeInput, GenerateCodeOutput, GenerateCodeTarget, Rust, Schema, TypeScript, GenerateCodeUseCase

### Community 63 - "schema_core"
Cohesion: 0.22
Nodes (5): list_schema_files(), load_directory_schema(), SchemaFileSource, list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema()

### Community 64 - "VectorDist"
Cohesion: 0.29
Nodes (4): VectorDist, Cosine, Euclidean, Manhattan

### Community 65 - "Infrastructure Layer"
Cohesion: 0.19
Nodes (7): resolveExitCode, AppLogger, Error Catalog, Orchestrator, Shared Capability Package, resolve_exit_code, Exit Codes

### Community 66 - "MigrationLedger"
Cohesion: 0.14
Nodes (9): AppliedMigration, MigrationLedger, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending (+1 more)

### Community 67 - "require_surrealdb"
Cohesion: 0.33
Nodes (8): docker_unavailable(), require_surrealdb(), schema_with_endpoint(), start_surrealdb(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_applies_schema_to_surrealdb(), cli_migrate_apply_status_and_rollback()

### Community 74 - "format_schema.rs"
Cohesion: 0.52
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

## Knowledge Gaps
- **130 isolated node(s):** `CONFIG_FILE`, `TABLES_DIR`, `TABLE_SUFFIX`, `MINIMAL_SCHEMA`, `MINIMAL_SCHEMA` (+125 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 403 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **60 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `MigrationLedger`, `core`, `arc`, `common/mod.rs`, `format_schema.rs`, `domain.rs`, `DatabaseSchema`, `migrate_dev.rs`, `DatabaseConfig`, `MigrationStore`, `SchemaSource`, `generate_code.rs`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CONFIG_FILE`, `TABLES_DIR`, `TABLE_SUFFIX` to the rest of the system?**
  _130 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FieldType` be split into smaller, more focused modules?**
  _Cohesion score 0.12727272727272726 - nodes in this community are weakly interconnected._
- **Why does `DatabaseConfig` connect `DatabaseConfig` to `MigrationLedger`, `arc`, `common/mod.rs`, `StubIntrospector`, `src/print.rs`, `usecases.rs`, `DatabaseSchema`, `database_config.rs`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Should `MigrationOperation` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._
- **Should `Commands` be split into smaller, more focused modules?**
  _Cohesion score 0.08831908831908832 - nodes in this community are weakly interconnected._