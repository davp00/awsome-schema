# Graph Report - awsome-schema  (2026-10-08)

## Corpus Check
- 158 files · ~90,921 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 4, .xml 3, .schema 2)

## Summary
- 1688 nodes · 3683 edges · 123 communities (61 shown, 62 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8f503907`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FieldType
- parser/src/lib.rs
- Rust Best Practices
- surrealdb_migration_ledger.rs
- codegen-typescript/tests/generator.rs
- Printer
- dispatch.rs
- cli crate
- coverage_gaps.rs
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- DatabaseSchema
- arc
- migrate_status.rs
- validation.rs
- AppContext
- Milestone 1 Offline Schema Toolchain
- naming.rs
- Performance Mindset
- cli
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- normalize.rs
- schema_core
- setupClient.ts
- templates.rs
- check.sh
- install-hooks.sh
- Status Check 2026-10-06
- codegen-typescript/src/lib.rs
- DatabaseConfig
- Awesome Feature — gate checklist
- cargo clippy
- Understanding Pointers
- domain.rs
- src/print.rs
- Error Handling
- Comments versus Documentation
- Split-Core Hexagonal CLI Architecture
- common/mod.rs
- FsAdapter
- migrate_dev.rs
- MigrationOperation
- SchemaSource
- DomainError
- globalSetup.ts
- manual-test-indexes.sh
- generate_code.rs
- tests/db_pull.rs
- NamingConvention
- compilerOptions
- require_surrealdb
- MigrationStoreAdapter
- VectorDist
- TestProject
- btreemap
- Infrastructure Layer
- core crate
- commands.rs
- coverage.sh
- package.json
- surrealAdapter.ts
- database_config.rs
- surrealdb_executor.rs
- live.test.ts
- Client Generators
- format.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 71 edges
2. `map_database_info()` - 41 edges
3. `Model` - 37 edges
4. `FieldType` - 36 edges
5. `validate_schema()` - 36 edges
6. `emit_typescript()` - 34 edges
7. `AppContext` - 34 edges
8. `sample_schema()` - 33 edges
9. `Field` - 32 edges
10. `DatabaseConfig` - 31 edges

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

## Communities (123 total, 62 thin omitted)

### Community 0 - "FieldType"
Cohesion: 0.12
Nodes (18): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+10 more)

### Community 1 - "parser/src/lib.rs"
Cohesion: 0.18
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "surrealdb_migration_ledger.rs"
Cohesion: 0.21
Nodes (8): ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql(), SurrealDbMigrationLedger, with_runtime()

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.07
Nodes (31): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby() (+23 more)

### Community 5 - "Printer"
Cohesion: 0.14
Nodes (8): run(), run_apply(), run_create(), run_dev(), run_rollback(), run_status(), run(), Printer

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (38): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+30 more)

### Community 7 - "cli crate"
Cohesion: 0.17
Nodes (12): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, ImportUsersUseCase, mockall, UserRepository, AppContext (+4 more)

### Community 8 - "coverage_gaps.rs"
Cohesion: 0.13
Nodes (12): test_context_with_introspector(), init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, OkExecutor, run_pull_warns_when_lossy() (+4 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.06
Nodes (35): db_push_renders_and_applies_schema(), EmptyDiff, format_schema_multiple_files_without_write_back(), format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), format_schema_writes_back_multiple_files(), generate_code_targets_schema_rust_and_typescript(), generate_typescript_writes_generator_output_unless_stdout() (+27 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "migrations/src/lib.rs"
Cohesion: 0.22
Nodes (26): creates_indexes_when_model_is_new(), creates_initial_migration_from_empty_snapshot(), creates_permission_on_new_edge(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions() (+18 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (30): NamingContext, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table() (+22 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.15
Nodes (9): introspect_async(), SurrealDbIntrospector, DatabaseSchema, merge_pulled_schema(), MigrationStore, SchemaIntrospector, DbPullInput, DbPullOutput (+1 more)

### Community 17 - "migrate_status.rs"
Cohesion: 0.29
Nodes (7): MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 18 - "validation.rs"
Cohesion: 0.16
Nodes (36): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), rejects_computed_link_with_on_delete() (+28 more)

### Community 19 - "AppContext"
Cohesion: 0.11
Nodes (3): AppContext, build_context(), build_context_with_introspector()

### Community 20 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.15
Nodes (18): Milestone 1 Offline Schema Toolchain, Post-Plan Work on main, Roadmap Notes, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, SurrealDB Rust SDK Upgrade (+10 more)

### Community 21 - "naming.rs"
Cohesion: 0.09
Nodes (18): map_attribute_overrides_convention(), map_field_path(), mapped_name(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingCase, CamelCase, KebabCase (+10 more)

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
Cohesion: 0.14
Nodes (20): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, UserDatabaseRepository (+12 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.13
Nodes (41): Field, Model, LinkStorage, Computed, Stored, bare_field(), empty_base_schema(), ensure_on_delete_default() (+33 more)

### Community 28 - "schema_core"
Cohesion: 0.22
Nodes (5): list_schema_files(), load_directory_schema(), SchemaFileSource, list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema()

### Community 30 - "setupClient.ts"
Cohesion: 0.20
Nodes (6): connectSurreal(), openGeneratedClient(), readE2eEnv(), recordIdString(), deleteByEmail(), vitest

### Community 36 - "Status Check 2026-10-06"
Cohesion: 0.15
Nodes (19): Real Graph Edges, Index Kinds, Migration Lifecycle, SchemaDiffer, db push, DropIndex Gap, edge Block Gap, Index Flag Gap (+11 more)

### Community 37 - "codegen-typescript/src/lib.rs"
Cohesion: 0.10
Nodes (67): client_key(), edge_groupby_numeric_fields(), edge_groupby_scalar_fields(), emit_build_order_by(), emit_build_where(), emit_crud_helpers(), emit_edge_aggregate_types(), emit_edge_inputs() (+59 more)

### Community 38 - "DatabaseConfig"
Cohesion: 0.18
Nodes (5): EmptyLedger, StatusLedger, DatabaseConfig, AppliedMigration, MigrationLedger

### Community 40 - "Awesome Feature — gate checklist"
Cohesion: 0.10
Nodes (18): Architecture, Awesome Feature — gate checklist, Docs / memory / git, Generated-client e2e, Plan, Rust e2e, Unit / coverage, Awesome Feature (+10 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "domain.rs"
Cohesion: 0.09
Nodes (13): Datasource, DatabaseExecutor, SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase, checksum_of(), MigrateApplyInput (+5 more)

### Community 44 - "src/print.rs"
Cohesion: 0.06
Nodes (38): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), NoopPull, run_pull(), run_push(), write_single_schema(), write_split_schema() (+30 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.15
Nodes (15): CLI Architecture Checklist, resolveExitCode, CLI Architecture Reference, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError (+7 more)

### Community 49 - "common/mod.rs"
Cohesion: 0.13
Nodes (10): connect(), ws_connection_address(), connect(), field_define(), MINIMAL_SCHEMA, record_ref_schema(), RECORD_REF_SCHEMA_TEMPLATE, table_exists() (+2 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.27
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "MigrationOperation"
Cohesion: 0.12
Nodes (17): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+9 more)

### Community 55 - "SchemaSource"
Cohesion: 0.13
Nodes (11): SchemaSource, FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace(), MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase (+3 more)

### Community 56 - "DomainError"
Cohesion: 0.09
Nodes (16): DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError, RenderError, SchemaNotFound (+8 more)

### Community 57 - "globalSetup.ts"
Cohesion: 0.18
Nodes (12): formatError(), globalSetup(), runCli(), writeEnv(), E2eEnv, envPath, findWorkspaceRoot(), fixtureSchemaPath (+4 more)

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "generate_code.rs"
Cohesion: 0.15
Nodes (15): CodeGeneratorPort, DirAwareFs, GenerateCodeInput, GenerateCodeOutput, GenerateCodeTarget, Rust, Schema, TypeScript (+7 more)

### Community 63 - "NamingConvention"
Cohesion: 0.31
Nodes (7): NamingConvention, diff_edge(), diff_model(), field_operations_for_edge(), field_operations_for_model(), index_operations_for_model(), relation_endpoints()

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "require_surrealdb"
Cohesion: 0.33
Nodes (8): docker_unavailable(), require_surrealdb(), schema_with_endpoint(), start_surrealdb(), cli_db_pull_overwrites_schema_after_push(), cli_db_pull_split_by_table_writes_schema_directory(), cli_db_push_applies_schema_to_surrealdb(), cli_migrate_apply_status_and_rollback()

### Community 67 - "VectorDist"
Cohesion: 0.29
Nodes (4): VectorDist, Cosine, Euclidean, Manhattan

### Community 69 - "btreemap"
Cohesion: 0.08
Nodes (12): OnDeleteAction, Cascade, Ignore, Reject, Unset, Edge, Model, TableMode (+4 more)

### Community 70 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 74 - "core crate"
Cohesion: 0.23
Nodes (11): db pull, Later Backlog, cli crate, Connectivity Stubs, core crate, MigrationStore, Planned Database Providers, renderers crate (+3 more)

### Community 83 - "package.json"
Cohesion: 0.12
Nodes (16): devDependencies, surrealdb, testcontainers, @types/node, typescript, vitest, name, private (+8 more)

### Community 101 - "surrealAdapter.ts"
Cohesion: 0.14
Nodes (10): asSurrealLike(), asSurrealTransaction(), bindOps(), encodeDeep(), LiveAction, LiveHandle, SurrealLike, SurrealOpsLike (+2 more)

### Community 121 - "database_config.rs"
Cohesion: 0.26
Nodes (9): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), preserves_wss_endpoint(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+1 more)

### Community 122 - "surrealdb_executor.rs"
Cohesion: 0.53
Nodes (4): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor

### Community 123 - "live.test.ts"
Cohesion: 0.33
Nodes (3): deleteByEmail(), LiveAction, LiveHandle

### Community 124 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

### Community 125 - "format.rs"
Cohesion: 0.18
Nodes (3): run(), run(), dispatch()

## Knowledge Gaps
- **180 isolated node(s):** `FIXTURE`, `LiveAction`, `LiveHandle`, `SurrealLike`, `SurrealOpsLike` (+175 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 484 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **62 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `surrealdb_migration_ledger.rs`, `codegen-typescript/tests/generator.rs`, `DatabaseConfig`, `domain.rs`, `DatabaseSchema`, `common/mod.rs`, `migrate_status.rs`, `migrate_dev.rs`, `SchemaSource`, `surrealdb_executor.rs`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `FIXTURE`, `LiveAction`, `LiveHandle` to the rest of the system?**
  _180 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FieldType` be split into smaller, more focused modules?**
  _Cohesion score 0.11747516072472239 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `coverage_gaps.rs`, `Printer`, `generate_code.rs`, `format.rs`?**
  _High betweenness centrality (0.032) - this node is a cross-community bridge._
- **Should `codegen-typescript/tests/generator.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0663265306122449 - nodes in this community are weakly interconnected._
- **Should `Printer` be split into smaller, more focused modules?**
  _Cohesion score 0.14210526315789473 - nodes in this community are weakly interconnected._