# Graph Report - awsome-schema  (2026-10-07)

## Corpus Check
- 153 files · ~85,177 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 4, .xml 3, .schema 2)

## Summary
- 1662 nodes · 3618 edges · 118 communities (57 shown, 61 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2951cb4b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Parser
- parser/src/lib.rs
- Rust Best Practices
- arc
- codegen-typescript/tests/generator.rs
- Printer
- dispatch.rs
- cli crate
- DatabaseConfig
- Automated Testing
- usecases.rs
- Token
- migrations/src/lib.rs
- surrealdb/mod.rs
- DatabaseSchema
- MigrationStore
- coverage_gaps.rs
- validation.rs
- AppContext
- Post-Plan Work on main
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
- SurrealDB-First Policy
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
- OnDeleteAction
- manual-test-indexes.sh
- generate_code.rs
- tests/db_pull.rs
- mapped_name
- compilerOptions
- NamingCase
- Milestone 1 Offline Schema Toolchain
- VectorDist
- generate.rs
- core crate
- commands.rs
- coverage.sh
- btreemap
- model.rs
- format_schema.rs
- Client Generators

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

## Communities (118 total, 61 thin omitted)

### Community 0 - "Parser"
Cohesion: 0.20
Nodes (6): link_target_from_type(), parse_index_fields(), parse_schema(), ParsedFieldAttributes, Parser, validate_field_rules()

### Community 1 - "parser/src/lib.rs"
Cohesion: 0.08
Nodes (19): map_field_type(), RustGenerator, maps_all_field_types_to_rust(), schema_with_field(), CodeGenerator, EXAMPLE, parse(), parses_edge_block() (+11 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "arc"
Cohesion: 0.31
Nodes (6): MigrateStatusInput, MigrateStatusOutput, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.13
Nodes (23): computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby(), emits_create_update_inputs_without_computed_fields(), emits_dual_shapes_for_models_and_edges(), emits_find_unique_hybrid_where(), emits_fluent_create_client(), emits_get_payload_and_select_types() (+15 more)

### Community 5 - "Printer"
Cohesion: 0.12
Nodes (8): run(), run_apply(), run_create(), run_dev(), run_rollback(), run_status(), run(), Printer

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (38): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+30 more)

### Community 7 - "cli crate"
Cohesion: 0.22
Nodes (10): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, anyhow, AppContext, AppContext, clap, cli crate (+2 more)

### Community 8 - "DatabaseConfig"
Cohesion: 0.07
Nodes (25): execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE (+17 more)

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
Cohesion: 0.15
Nodes (33): NamingConvention, creates_indexes_when_model_is_new(), creates_initial_migration_from_empty_snapshot(), creates_permission_on_new_edge(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode() (+25 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (30): NamingContext, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table() (+22 more)

### Community 15 - "DatabaseSchema"
Cohesion: 0.17
Nodes (10): introspect_async(), SurrealDbIntrospector, DatabaseSchema, Datasource, Generator, merge_pulled_schema(), SchemaIntrospector, DbPullInput (+2 more)

### Community 16 - "MigrationStore"
Cohesion: 0.12
Nodes (11): DatabaseExecutor, MigrationLedger, MigrationStore, checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase, MigrateRollbackInput (+3 more)

### Community 17 - "coverage_gaps.rs"
Cohesion: 0.13
Nodes (12): test_context_with_introspector(), init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, OkExecutor, run_pull_warns_when_lossy() (+4 more)

### Community 18 - "validation.rs"
Cohesion: 0.16
Nodes (36): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), rejects_computed_link_with_on_delete() (+28 more)

### Community 19 - "AppContext"
Cohesion: 0.11
Nodes (3): AppContext, build_context(), build_context_with_introspector()

### Community 20 - "Post-Plan Work on main"
Cohesion: 0.29
Nodes (10): Post-Plan Work on main, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, Awesome Schema DSL, Field Assignments, Flexible Object (+2 more)

### Community 21 - "naming.rs"
Cohesion: 0.20
Nodes (8): map_attribute_overrides_convention(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), split_identifier_parts(), to_camel_case(), to_kebab_case(), to_pascal_case(), to_snake_case()

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.39
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "introspect.rs"
Cohesion: 0.06
Nodes (72): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+64 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.07
Nodes (30): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, resolveExitCode, User (+22 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.13
Nodes (41): Field, Model, LinkStorage, Computed, Stored, bare_field(), empty_base_schema(), ensure_on_delete_default() (+33 more)

### Community 28 - "schema_core"
Cohesion: 0.20
Nodes (6): list_schema_files(), load_directory_schema(), SchemaFileSource, run(), list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema()

### Community 30 - "setupClient.ts"
Cohesion: 0.06
Nodes (41): formatError(), globalSetup(), runCli(), writeEnv(), devDependencies, surrealdb, testcontainers, @types/node (+33 more)

### Community 36 - "Status Check 2026-10-06"
Cohesion: 0.36
Nodes (9): Migration Lifecycle, db push, DropIndex Gap, migrate apply Gap, README Drift, Status Check 2026-10-06, MigrationOperation, Migration Workflow (+1 more)

### Community 37 - "codegen-typescript/src/lib.rs"
Cohesion: 0.10
Nodes (63): client_key(), edge_groupby_numeric_fields(), edge_groupby_scalar_fields(), emit_build_order_by(), emit_build_where(), emit_crud_helpers(), emit_edge_aggregate_types(), emit_edge_inputs() (+55 more)

### Community 38 - "SurrealDB-First Policy"
Cohesion: 0.20
Nodes (10): db pull, Real Graph Edges, Index Kinds, SchemaDiffer, edge Block Gap, Index Flag Gap, @relation Gap, HNSW Vector Index (+2 more)

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

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.23
Nodes (10): CLI Architecture Checklist, CLI Architecture Reference, awesome-schema binary, Cargo Workspace Layout, DomainError, Rust CLI Examples, Rust Implementation Guide, thiserror (+2 more)

### Community 49 - "common/mod.rs"
Cohesion: 0.06
Nodes (22): MigrationStoreAdapter, connect(), ws_connection_address(), connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA (+14 more)

### Community 51 - "migrate_dev.rs"
Cohesion: 0.30
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 54 - "MigrationOperation"
Cohesion: 0.11
Nodes (17): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+9 more)

### Community 55 - "SchemaSource"
Cohesion: 0.15
Nodes (7): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "DomainError"
Cohesion: 0.09
Nodes (16): DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError, RenderError, SchemaNotFound (+8 more)

### Community 57 - "OnDeleteAction"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "generate_code.rs"
Cohesion: 0.15
Nodes (15): CodeGeneratorPort, DirAwareFs, GenerateCodeInput, GenerateCodeOutput, GenerateCodeTarget, Rust, Schema, TypeScript (+7 more)

### Community 63 - "mapped_name"
Cohesion: 0.24
Nodes (3): map_field_path(), mapped_name(), NamingContext<'a>

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "NamingCase"
Cohesion: 0.25
Nodes (7): NamingCase, CamelCase, KebabCase, Lowercase, PascalCase, SnakeCase, naming_case_str()

### Community 66 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.29
Nodes (8): Milestone 1 Offline Schema Toolchain, Roadmap Notes, SurrealDB Rust SDK Upgrade, SurrealDB 3.3.0, Awesome Schema, Prisma, SurrealDB, TypeORM

### Community 67 - "VectorDist"
Cohesion: 0.29
Nodes (4): VectorDist, Cosine, Euclidean, Manhattan

### Community 74 - "core crate"
Cohesion: 0.26
Nodes (11): Later Backlog, cli crate, Connectivity Stubs, core crate, MigrationRenderer, MigrationStore, Planned Database Providers, renderers crate (+3 more)

### Community 121 - "model.rs"
Cohesion: 0.32
Nodes (5): Edge, Model, TableMode, Schemafull, Schemaless

### Community 122 - "format_schema.rs"
Cohesion: 0.52
Nodes (4): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace()

### Community 124 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

## Knowledge Gaps
- **176 isolated node(s):** `FIXTURE`, `SurrealLike`, `SurrealOpsLike`, `SurrealQueryable`, `SurrealTransactionLike` (+171 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 472 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **61 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `parser/src/lib.rs`, `arc`, `DatabaseConfig`, `domain.rs`, `DatabaseSchema`, `MigrationStore`, `common/mod.rs`, `migrate_dev.rs`, `SchemaSource`, `format_schema.rs`?**
  _High betweenness centrality (0.058) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `FIXTURE`, `SurrealLike`, `SurrealOpsLike` to the rest of the system?**
  _176 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `parser/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08108108108108109 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `generate.rs`, `Printer`, `coverage_gaps.rs`, `schema_core`, `generate_code.rs`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Should `codegen-typescript/tests/generator.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12698412698412698 - nodes in this community are weakly interconnected._
- **Why does `Field` connect `normalize.rs` to `Parser`, `model.rs`, `btreemap`, `src/print.rs`, `surrealdb/mod.rs`, `validation.rs`, `MigrationOperation`, `introspect.rs`, `OnDeleteAction`, `mapped_name`?**
  _High betweenness centrality (0.027) - this node is a cross-community bridge._