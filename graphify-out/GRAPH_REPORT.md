# Graph Report - awsome-schema  (2026-10-08)

## Corpus Check
- 179 files · ~101,840 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 15 file(s) not represented in the graph (top: (none) 5, .xml 3, .schema 2)

## Summary
- 1936 nodes · 4313 edges · 97 communities (80 shown, 17 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 148 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0ea1c4eb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Parser
- DatabaseConfig
- Rust Best Practices
- DatabaseSchema
- codegen-typescript/tests/generator.rs
- AppContext
- dispatch.rs
- cli crate
- coverage_gaps.rs
- Automated Testing
- usecases.rs
- Token
- javascript/package.json
- surrealdb/mod.rs
- runtime.ts
- index.ts
- assertReturnSelectExclusive
- validation.rs
- di.rs
- Post-Plan Work on main
- naming.rs
- Performance Mindset
- cli
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- normalize.rs
- generate_code.rs
- setupClient.ts
- templates.rs
- check.sh
- install-hooks.sh
- SurrealDB-First Policy
- migrations/src/lib.rs
- migrate_status.rs
- buildClientDelegates
- Awesome Feature — gate checklist
- cargo clippy
- Understanding Pointers
- updateWithNested
- src/print.rs
- Error Handling
- Comments versus Documentation
- normalizeThing
- Split-Core Hexagonal CLI Architecture
- common/mod.rs
- DomainError
- normalizeThing
- MigrationOperation
- SchemaSource
- FileSystemPort
- globalSetup.ts
- surrealAdapter.ts
- db.rs
- manual-test-indexes.sh
- src/surreal.rs
- MigrationStore
- compilerOptions
- buildWhere
- domain.rs
- Index
- findManyRecords
- btreemap
- parser/src/lib.rs
- buildWhere
- compilerOptions
- live.test.ts
- core crate
- deleteRecord
- Contributing
- field.rs
- coverage.sh
- applyNestedWrites
- findManyRecords
- arc
- typescript/package.json
- list_schema_files
- bug_report.md
- feature_request.md
- PULL_REQUEST_TEMPLATE.md
- SECURITY.md
- runtime.test.ts
- Status Check 2026-10-06
- package.json
- Client Generators
- Index Kinds
- database_config.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 178 edges
2. `DatabaseSchema` - 113 edges
3. `NamingContext` - 46 edges
4. `Field` - 43 edges
5. `map_database_info()` - 41 edges
6. `FieldType` - 40 edges
7. `AppContext` - 39 edges
8. `DatabaseConfig` - 37 edges
9. `FileSystemPort` - 36 edges
10. `validate_schema()` - 36 edges

## Surprising Connections (you probably didn't know these)
- `Generated-client e2e (any language)` --references--> `createClient()`  [INFERRED]
  .agents/skills/awesome-feature/SKILL.md → e2e/typescript/generated/index.ts
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `awesome-schema binary` --conceptually_related_to--> `cli crate`  [INFERRED]
  .agents/skills/split-core-hexagonal-cli/references/RUST.md → README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Awesome Schema Provider Ports** — readme_schema_renderer, readme_migration_renderer, readme_schema_introspector, readme_migration_store, readme_schema_source [EXTRACTED 1.00]
- **Faithful SurrealDB Schema Loop** — docs_roadmap_next_surrealdb_first, docs_roadmap_next_graph_edges, docs_roadmap_next_migration_lifecycle, docs_roadmap_next_db_pull, docs_roadmap_next_index_kinds [EXTRACTED 1.00]
- **Library and Binary Error Strategy** — agents_skills_rust_best_practices_references_chapter_04_result_over_panic, agents_skills_rust_best_practices_references_chapter_04_thiserror, agents_skills_rust_best_practices_references_chapter_04_anyhow, agents_skills_rust_best_practices_references_chapter_04_question_mark [EXTRACTED 1.00]
- **Rust Test Layers** — agents_skills_rust_best_practices_references_chapter_05_unit_tests, agents_skills_rust_best_practices_references_chapter_05_integration_tests, agents_skills_rust_best_practices_references_chapter_05_doc_tests [EXTRACTED 1.00]
- **Split-Core Architectural Layers** — agents_skills_split_core_hexagonal_cli_references_reference_domain_core, agents_skills_split_core_hexagonal_cli_references_reference_application_core, agents_skills_split_core_hexagonal_cli_references_reference_infrastructure_layer, agents_skills_split_core_hexagonal_cli_references_reference_shared_capability [EXTRACTED 1.00]
- **Static and Dynamic Dispatch Trade-off** — agents_skills_rust_best_practices_references_chapter_06_generics, agents_skills_rust_best_practices_references_chapter_06_static_dispatch, agents_skills_rust_best_practices_references_chapter_06_dynamic_dispatch [EXTRACTED 1.00]

## Communities (97 total, 17 thin omitted)

### Community 0 - "Parser"
Cohesion: 0.22
Nodes (6): link_target_from_type(), parse_index_fields(), parse_schema(), ParsedFieldAttributes, Parser, validate_field_rules()

### Community 1 - "DatabaseConfig"
Cohesion: 0.05
Nodes (23): introspect_async(), SurrealDbIntrospector, connect(), execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, ensure_schema_reports_connection_errors() (+15 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "DatabaseSchema"
Cohesion: 0.11
Nodes (56): StubIntrospector, edge_groupby_numeric_fields(), edge_groupby_scalar_fields(), emit_edge_aggregate_types(), emit_edge_inputs(), emit_edge_order_by_input(), emit_edge_record(), emit_edge_select_payload() (+48 more)

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.10
Nodes (26): computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby(), emits_create_update_inputs_without_computed_fields(), emits_dual_shapes_for_models_and_edges(), emits_find_unique_hybrid_where(), emits_fluent_create_client(), emits_get_payload_and_select_types() (+18 more)

### Community 5 - "AppContext"
Cohesion: 0.12
Nodes (12): run_push(), run(), run(), run_apply(), run_create(), run_dev(), run_rollback(), run_status() (+4 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (38): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+30 more)

### Community 7 - "cli crate"
Cohesion: 0.22
Nodes (10): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, anyhow, AppContext, AppContext, clap, cli crate (+2 more)

### Community 8 - "coverage_gaps.rs"
Cohesion: 0.16
Nodes (11): test_context_with_introspector(), init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, OkExecutor, run_pull_warns_when_lossy() (+3 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.12
Nodes (32): db_push_refuses_unknown_provider_before_render(), db_push_renders_and_applies_schema(), EmptyDiff, format_schema_multiple_files_without_write_back(), format_schema_trims_trailing_blank_lines(), format_schema_without_write_back(), format_schema_writes_back_multiple_files(), generate_code_targets_schema_rust_and_typescript() (+24 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "javascript/package.json"
Cohesion: 0.06
Nodes (37): child, require, { version }, assetName(), cachePath(), downloadBinary(), ensureBinary(), releaseUrl() (+29 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.16
Nodes (31): TableMode, Schemafull, Schemaless, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index() (+23 more)

### Community 15 - "runtime.ts"
Cohesion: 0.09
Nodes (22): bindNested(), FieldWriteMeta, GroupByAlias, insertStoredCreates(), LiveAction, LiveHandle, ManyReturnResult, MutationReturn (+14 more)

### Community 16 - "index.ts"
Cohesion: 0.02
Nodes (84): AggregateFilter, ArrayFilter, BooleanFilter, DateFilter, FieldSelectMeta, FieldWriteMeta, GroupByAlias, IdFilter (+76 more)

### Community 17 - "assertReturnSelectExclusive"
Cohesion: 0.26
Nodes (14): asRows(), assertNonEmptyWhere(), assertReturnSelectExclusive(), buildProjection(), countMatching(), createManyVia(), deleteManyRecords(), deleteOneRecord() (+6 more)

### Community 18 - "validation.rs"
Cohesion: 0.17
Nodes (36): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), rejects_computed_link_with_on_delete() (+28 more)

### Community 20 - "Post-Plan Work on main"
Cohesion: 0.29
Nodes (10): Post-Plan Work on main, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, Awesome Schema DSL, Field Assignments, Flexible Object (+2 more)

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
Cohesion: 0.05
Nodes (73): maps_all_field_types_to_rust(), rejects_unimplemented_rust_provider(), schema_with_field(), FieldType, Array, Bool, Custom, Datetime (+65 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.07
Nodes (30): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, resolveExitCode, User (+22 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.15
Nodes (41): LinkStorage, Computed, Stored, ObjectTypeDefinition, ObjectTypeField, bare_field(), empty_base_schema(), ensure_on_delete_default() (+33 more)

### Community 28 - "generate_code.rs"
Cohesion: 0.14
Nodes (16): CodeGeneratorPort, DirAwareFs, GenerateCodeInput, GenerateCodeOutput, GenerateCodeTarget, Rust, Schema, TypeScript (+8 more)

### Community 30 - "setupClient.ts"
Cohesion: 0.18
Nodes (6): connectSurreal(), openGeneratedClient(), readE2eEnv(), recordIdString(), asSurrealLike(), deleteByEmail()

### Community 36 - "SurrealDB-First Policy"
Cohesion: 0.19
Nodes (12): db pull, Later Backlog, Milestone 1 Offline Schema Toolchain, Roadmap Notes, SurrealDB Rust SDK Upgrade, SurrealDB 3.3.0, Awesome Schema, Connectivity Stubs (+4 more)

### Community 37 - "migrations/src/lib.rs"
Cohesion: 0.24
Nodes (26): creates_indexes_when_model_is_new(), creates_initial_migration_from_empty_snapshot(), creates_permission_on_new_edge(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions() (+18 more)

### Community 38 - "migrate_status.rs"
Cohesion: 0.31
Nodes (6): MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 39 - "buildClientDelegates"
Cohesion: 0.15
Nodes (24): asRows(), assertNonEmptyWhere(), assertReturnSelectExclusive(), buildClientDelegates(), buildProjection(), countMatching(), createClient(), createManyVia() (+16 more)

### Community 40 - "Awesome Feature — gate checklist"
Cohesion: 0.10
Nodes (18): Architecture, Awesome Feature — gate checklist, Docs / memory / git, Generated-client e2e, Plan, Rust e2e, Unit / coverage, Awesome Feature (+10 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "updateWithNested"
Cohesion: 0.14
Nodes (20): applyNestedWrites(), assertNoNestedWrites(), bindNested(), collectComputed(), createPost(), createUser(), createWithNested(), insertStoredCreates() (+12 more)

### Community 44 - "src/print.rs"
Cohesion: 0.14
Nodes (25): field_type_for_print(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field(), print_field_type(), print_generator() (+17 more)

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "normalizeThing"
Cohesion: 0.13
Nodes (20): collectEdge(), createLikes(), edgeContent(), edgeFarData(), findUniqueRecord(), firstRow(), normalizeThing(), relateEdge() (+12 more)

### Community 48 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.23
Nodes (10): CLI Architecture Checklist, CLI Architecture Reference, awesome-schema binary, Cargo Workspace Layout, DomainError, Rust CLI Examples, Rust Implementation Guide, thiserror (+2 more)

### Community 49 - "common/mod.rs"
Cohesion: 0.06
Nodes (21): MINIMAL_SCHEMA, UNREACHABLE_SCHEMA, connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA, record_ref_schema() (+13 more)

### Community 50 - "DomainError"
Cohesion: 0.07
Nodes (17): FsAdapter, DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError, RenderError (+9 more)

### Community 51 - "normalizeThing"
Cohesion: 0.23
Nodes (13): collectEdge(), deleteRecord(), edgeContent(), edgeFarData(), findUniqueRecord(), firstRow(), normalizeThing(), relateEdge() (+5 more)

### Community 54 - "MigrationOperation"
Cohesion: 0.11
Nodes (25): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+17 more)

### Community 55 - "SchemaSource"
Cohesion: 0.17
Nodes (8): SchemaSource, FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace(), ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "FileSystemPort"
Cohesion: 0.11
Nodes (7): MigrationStoreAdapter, load_directory_schema(), SchemaFileSource, FileSystemPort, InitProjectInput, InitProjectOutput, InitProjectUseCase

### Community 57 - "globalSetup.ts"
Cohesion: 0.20
Nodes (12): formatError(), globalSetup(), runCli(), writeEnv(), E2eEnv, envPath, findWorkspaceRoot(), fixtureSchemaPath (+4 more)

### Community 58 - "surrealAdapter.ts"
Cohesion: 0.14
Nodes (9): asSurrealTransaction(), bindOps(), encodeDeep(), LiveAction, LiveHandle, SurrealLike, SurrealOpsLike, SurrealQueryable (+1 more)

### Community 59 - "db.rs"
Cohesion: 0.24
Nodes (11): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), run_pull(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir() (+3 more)

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "src/surreal.rs"
Cohesion: 0.08
Nodes (21): map_field_type(), RustGenerator, CodeGenerator, SURREAL, typescript_provider(), TypeScriptGenerator, TypeScriptProvider, client_key() (+13 more)

### Community 63 - "MigrationStore"
Cohesion: 0.13
Nodes (10): require_surreal_provider(), MigrationStore, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase (+2 more)

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "buildWhere"
Cohesion: 0.18
Nodes (12): assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildWhere(), countRecords(), groupByRecords(), isScalarFilterObject(), listRelationPredicate() (+4 more)

### Community 66 - "domain.rs"
Cohesion: 0.08
Nodes (14): Datasource, DatabaseExecutor, SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase, checksum_of(), MigrateApplyInput (+6 more)

### Community 67 - "Index"
Cohesion: 0.14
Nodes (6): Index, VectorDist, Cosine, Euclidean, Manhattan, Model

### Community 68 - "findManyRecords"
Cohesion: 0.31
Nodes (9): appendLimitStart(), buildOrderBy(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField(), relationCountExpr() (+1 more)

### Community 69 - "btreemap"
Cohesion: 0.18
Nodes (3): Relation, Generator, merge_pulled_schema()

### Community 70 - "parser/src/lib.rs"
Cohesion: 0.21
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 71 - "buildWhere"
Cohesion: 0.18
Nodes (12): assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildWhere(), countRecords(), groupByRecords(), isScalarFilterObject(), listRelationPredicate() (+4 more)

### Community 72 - "compilerOptions"
Cohesion: 0.17
Nodes (11): compilerOptions, declaration, module, moduleResolution, outDir, rootDir, skipLibCheck, strict (+3 more)

### Community 73 - "live.test.ts"
Cohesion: 0.33
Nodes (3): deleteByEmail(), LiveAction, LiveHandle

### Community 74 - "core crate"
Cohesion: 0.33
Nodes (8): cli crate, core crate, MigrationRenderer, MigrationStore, renderers crate, SchemaIntrospector, SchemaRenderer, SchemaSource

### Community 75 - "deleteRecord"
Cohesion: 0.50
Nodes (4): deleteLikes(), deletePost(), deleteRecord(), deleteUser()

### Community 76 - "Contributing"
Cohesion: 0.20
Nodes (8): awesome-schema, Adding a database provider, Coding agents, Contributing, Development, Prerequisites, Tests, Workspace

### Community 78 - "field.rs"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 80 - "applyNestedWrites"
Cohesion: 0.36
Nodes (10): applyNestedWrites(), assertNoNestedWrites(), collectComputed(), createWithNested(), isNestedWriteBag(), readStoredLink(), rowHasNestedWrite(), splitAndResolveWriteData() (+2 more)

### Community 81 - "findManyRecords"
Cohesion: 0.31
Nodes (9): appendLimitStart(), buildOrderBy(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField(), relationCountExpr() (+1 more)

### Community 82 - "arc"
Cohesion: 0.17
Nodes (6): NoopPull, StubIntrospector, SchemaIntrospector, DbPullInput, DbPullOutput, DbPullUseCase

### Community 83 - "typescript/package.json"
Cohesion: 0.10
Nodes (20): dependencies, awesome-schema, devDependencies, surrealdb, testcontainers, @types/node, typescript, vitest (+12 more)

### Community 84 - "list_schema_files"
Cohesion: 0.29
Nodes (4): list_schema_files(), run(), list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema()

### Community 85 - "bug_report.md"
Cohesion: 0.33
Nodes (5): Command, Schema snippet, SurrealDB version, What happened, What you expected

### Community 86 - "feature_request.md"
Cohesion: 0.50
Nodes (3): Command or surface, Provider, What you want

### Community 89 - "runtime.test.ts"
Cohesion: 0.29
Nodes (6): asRecordId(), bindSchemaMeta(), FieldSelectMeta, parseRecordId(), recordId, selectMeta

### Community 90 - "Status Check 2026-10-06"
Cohesion: 0.19
Nodes (15): Real Graph Edges, Migration Lifecycle, SchemaDiffer, db push, DropIndex Gap, edge Block Gap, migrate apply Gap, README Drift (+7 more)

### Community 91 - "package.json"
Cohesion: 0.33
Nodes (5): devDependencies, awesome-schema, awesome-schema, packageManager, private

### Community 94 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

### Community 95 - "Index Kinds"
Cohesion: 1.00
Nodes (3): Index Kinds, Index Flag Gap, HNSW Vector Index

### Community 118 - "database_config.rs"
Cohesion: 0.29
Nodes (9): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), preserves_wss_endpoint(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+1 more)

## Knowledge Gaps
- **339 isolated node(s):** `awesome-schema`, `name`, `description`, `license`, `type` (+334 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 578 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `Parser`, `DatabaseConfig`, `DatabaseSchema`, `AppContext`, `coverage_gaps.rs`, `usecases.rs`, `surrealdb/mod.rs`, `validation.rs`, `introspect.rs`, `normalize.rs`, `generate_code.rs`, `migrate_status.rs`, `SchemaSource`, `FileSystemPort`, `db.rs`, `src/surreal.rs`, `MigrationStore`, `domain.rs`, `parser/src/lib.rs`, `arc`, `list_schema_files`, `database_config.rs`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `awesome-schema`, `name`, `description` to the rest of the system?**
  _339 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DatabaseConfig` be split into smaller, more focused modules?**
  _Cohesion score 0.050921861281826165 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `Parser`, `DatabaseConfig`, `AppContext`, `coverage_gaps.rs`, `usecases.rs`, `surrealdb/mod.rs`, `validation.rs`, `introspect.rs`, `normalize.rs`, `generate_code.rs`, `migrations/src/lib.rs`, `src/print.rs`, `DomainError`, `MigrationOperation`, `SchemaSource`, `FileSystemPort`, `db.rs`, `src/surreal.rs`, `MigrationStore`, `domain.rs`, `btreemap`, `parser/src/lib.rs`, `arc`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `DatabaseSchema` be split into smaller, more focused modules?**
  _Cohesion score 0.1082693947144075 - nodes in this community are weakly interconnected._
- **Why does `Field` connect `DatabaseSchema` to `Parser`, `Index`, `btreemap`, `src/print.rs`, `field.rs`, `surrealdb/mod.rs`, `validation.rs`, `naming.rs`, `MigrationOperation`, `introspect.rs`, `normalize.rs`?**
  _High betweenness centrality (0.021) - this node is a cross-community bridge._