# Graph Report - awsome-schema  (2026-10-09)

## Corpus Check
- 184 files · ~104,694 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 15 file(s) not represented in the graph (top: (none) 5, .xml 3, .schema 2)

## Summary
- 2012 nodes · 4469 edges · 113 communities (89 shown, 24 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 152 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `10d08503`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Parser
- DatabaseConfig
- Rust Best Practices
- types.rs
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
- Milestone 1 Offline Schema Toolchain
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
- Status Check 2026-10-06
- migrations/src/lib.rs
- compile-client.cjs
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
- FieldType
- normalizeThing
- usecases/db_pull.rs
- SchemaSource
- DomainError
- globalSetup.ts
- surreal.ts
- db.rs
- manual-test-indexes.sh
- codegen-typescript/src/lib.rs
- DatabaseExecutor
- compilerOptions
- buildWhere
- arc
- parser/src/lib.rs
- findManyRecords
- field.rs
- Index
- core
- compilerOptions
- list_schema_files
- core crate
- deleteRecord
- Contributing
- lib.js
- coverage.sh
- applyNestedWrites
- buildProjection
- Infrastructure Layer
- typescript/package.json
- surrealdb_migration_ledger.rs
- bug_report.md
- feature_request.md
- PULL_REQUEST_TEMPLATE.md
- SECURITY.md
- MigrationOperation
- Migration Workflow
- package.json
- OnDeleteAction
- ./runtime
- DatabaseSchema
- parse.rs
- scripts
- parse_field_define
- buildWhere
- btreemap
- NamingConvention
- schema_core
- codegen-rust/tests/generator.rs
- devDependencies
- ObjectTypeField
- database_config.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 152 edges
2. `DatabaseSchema` - 70 edges
3. `map_database_info()` - 41 edges
4. `AppContext` - 39 edges
5. `DatabaseConfig` - 37 edges
6. `FieldType` - 36 edges
7. `validate_schema()` - 36 edges
8. `sample_schema()` - 33 edges
9. `Field` - 32 edges
10. `Parser` - 28 edges

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

## Communities (113 total, 24 thin omitted)

### Community 1 - "DatabaseConfig"
Cohesion: 0.11
Nodes (9): EmptyLedger, StatusLedger, DatabaseConfig, AppliedMigration, MigrationLedger, MigrationStore, MigrateApplyUseCase, MemoryLedger (+1 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "types.rs"
Cohesion: 0.09
Nodes (77): client_key(), emit_crud_helpers(), emit_fluent_client(), emit_fluent_edge_delegate(), emit_fluent_model_delegate(), emit_query_helpers(), emit_select_helpers(), emit_session_live() (+69 more)

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.09
Nodes (27): computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby(), emits_create_update_inputs_without_computed_fields(), emits_dual_shapes_for_models_and_edges(), emits_find_unique_hybrid_where(), emits_fluent_create_client(), emits_get_payload_and_select_types() (+19 more)

### Community 5 - "AppContext"
Cohesion: 0.14
Nodes (11): run_push(), run(), run_apply(), run_create(), run_dev(), run_rollback(), run_status(), dispatch() (+3 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (38): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+30 more)

### Community 7 - "cli crate"
Cohesion: 0.17
Nodes (12): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, ImportUsersUseCase, mockall, UserRepository, AppContext (+4 more)

### Community 8 - "coverage_gaps.rs"
Cohesion: 0.18
Nodes (10): test_context_with_introspector(), init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, run_pull_warns_when_lossy(), run_push_succeeds_with_stub_executor() (+2 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.05
Nodes (42): MigrateRollbackInput, MigrateRollbackOutput, MigrateRollbackUseCase, MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied (+34 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "javascript/package.json"
Cohesion: 0.10
Nodes (20): require, { version }, bin, awesome-schema, description, engines, node, files (+12 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.13
Nodes (31): MigrationPlan, NamingContext, MigrationRenderer, CountingRenderer, RecordingRenderer, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link() (+23 more)

### Community 15 - "runtime.ts"
Cohesion: 0.09
Nodes (29): asRecordId(), assertAggregateFilter(), bindNested(), bindSchemaMeta(), buildGroupByOrderBy(), buildGroupHaving(), FieldSelectMeta, FieldWriteMeta (+21 more)

### Community 16 - "index.ts"
Cohesion: 0.02
Nodes (84): AggregateFilter, ArrayFilter, BooleanFilter, DateFilter, FieldSelectMeta, FieldWriteMeta, GroupByAlias, IdFilter (+76 more)

### Community 17 - "assertReturnSelectExclusive"
Cohesion: 0.27
Nodes (13): asRows(), assertNonEmptyWhere(), assertReturnSelectExclusive(), countMatching(), createManyVia(), deleteManyRecords(), deleteOneRecord(), insertManyRows() (+5 more)

### Community 18 - "validation.rs"
Cohesion: 0.16
Nodes (36): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), rejects_computed_link_with_on_delete() (+28 more)

### Community 20 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.14
Nodes (18): Milestone 1 Offline Schema Toolchain, Post-Plan Work on main, db push, edge Block Gap, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement (+10 more)

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
Cohesion: 0.15
Nodes (31): computed_without_backlink_and_weird_body_are_not_backlinks(), ensure_model_id_field(), field_define_errors_propagate_through_map_fields(), id_field_non_record_keeps_scalar_type(), is_relation_table(), map_database_info(), maps_computed_backlink_single_table_and_field_form(), maps_field_modifiers_and_trailing_tokens() (+23 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.14
Nodes (20): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, UserDatabaseRepository (+12 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.13
Nodes (41): Field, Model, LinkStorage, Computed, Stored, bare_field(), empty_base_schema(), ensure_on_delete_default() (+33 more)

### Community 28 - "generate_code.rs"
Cohesion: 0.09
Nodes (25): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace(), CLIENT_PACKAGE_JSON, client_package_json_path(), CodeGeneratorPort, DirAwareFs (+17 more)

### Community 30 - "setupClient.ts"
Cohesion: 0.13
Nodes (8): connectSurreal(), openGeneratedClient(), readE2eEnv(), recordIdString(), deleteByEmail(), LiveAction, LiveHandle, deleteByEmail()

### Community 36 - "Status Check 2026-10-06"
Cohesion: 0.15
Nodes (14): awesome-schema, db pull, Real Graph Edges, Index Kinds, SchemaDiffer, Roadmap Notes, db pull Not Implemented, Index Flag Gap (+6 more)

### Community 37 - "migrations/src/lib.rs"
Cohesion: 0.23
Nodes (27): creates_indexes_when_model_is_new(), creates_initial_migration_from_empty_snapshot(), creates_permission_on_new_edge(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions() (+19 more)

### Community 38 - "compile-client.cjs"
Cohesion: 0.14
Nodes (15): child, require, { version }, bindings(), compileClientFile(), compileGeneratedClient(), { dirname, join, resolve, sep }, esmToCjs() (+7 more)

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
Cohesion: 0.15
Nodes (22): field_type_for_print(), print_config_blocks(), print_datasource(), print_edge(), print_edge_block(), print_field(), print_field_type(), print_model() (+14 more)

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
Cohesion: 0.16
Nodes (14): CLI Architecture Checklist, resolveExitCode, CLI Architecture Reference, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError (+6 more)

### Community 49 - "common/mod.rs"
Cohesion: 0.06
Nodes (21): MINIMAL_SCHEMA, UNREACHABLE_SCHEMA, connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA, record_ref_schema() (+13 more)

### Community 50 - "FieldType"
Cohesion: 0.14
Nodes (15): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+7 more)

### Community 51 - "normalizeThing"
Cohesion: 0.23
Nodes (13): collectEdge(), deleteRecord(), edgeContent(), edgeFarData(), findUniqueRecord(), firstRow(), normalizeThing(), relateEdge() (+5 more)

### Community 54 - "usecases/db_pull.rs"
Cohesion: 0.33
Nodes (4): merge_pulled_schema(), DbPullInput, DbPullOutput, DbPullUseCase

### Community 55 - "SchemaSource"
Cohesion: 0.11
Nodes (12): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort (+4 more)

### Community 56 - "DomainError"
Cohesion: 0.06
Nodes (20): FsAdapter, MigrationStoreAdapter, DomainError, CodegenError, DatabaseError, MigrationError, NotImplemented, ParseError (+12 more)

### Community 57 - "globalSetup.ts"
Cohesion: 0.22
Nodes (12): formatError(), globalSetup(), runCli(), writeEnv(), E2eEnv, envPath, findWorkspaceRoot(), fixtureSchemaPath (+4 more)

### Community 58 - "surreal.ts"
Cohesion: 0.12
Nodes (12): LiveHandle, SurrealLike, SurrealOpsLike, SurrealTransactionLike, ADAPTED, asSurrealLike(), asSurrealTransaction(), bindOps() (+4 more)

### Community 59 - "db.rs"
Cohesion: 0.24
Nodes (11): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), run_pull(), write_single_schema(), write_split_schema(), CONFIG_FILE, split_schema_dir() (+3 more)

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "codegen-typescript/src/lib.rs"
Cohesion: 0.18
Nodes (5): SURREAL, typescript_provider(), TypeScriptGenerator, TypeScriptProvider, SurrealTypeScriptProvider

### Community 63 - "DatabaseExecutor"
Cohesion: 0.19
Nodes (6): OkExecutor, DatabaseExecutor, SchemaRenderer, DbPushInput, DbPushOutput, DbPushUseCase

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "buildWhere"
Cohesion: 0.18
Nodes (12): assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildWhere(), countRecords(), groupByRecords(), isScalarFilterObject(), listRelationPredicate() (+4 more)

### Community 66 - "arc"
Cohesion: 0.14
Nodes (7): NoopPull, StubIntrospector, StubIntrospector, SchemaIntrospector, checksum_of(), MigrateApplyInput, MigrateApplyOutput

### Community 67 - "parser/src/lib.rs"
Cohesion: 0.21
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 68 - "findManyRecords"
Cohesion: 0.31
Nodes (9): appendLimitStart(), buildOrderBy(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField(), relationCountExpr() (+1 more)

### Community 69 - "field.rs"
Cohesion: 0.13
Nodes (9): RelationEndpoints, Model, TableMode, Schemafull, Schemaless, Relation, render_define_relation_table(), render_define_table() (+1 more)

### Community 70 - "Index"
Cohesion: 0.17
Nodes (5): Index, VectorDist, Cosine, Euclidean, Manhattan

### Community 71 - "core"
Cohesion: 0.24
Nodes (3): map_field_type(), RustGenerator, CodeGenerator

### Community 72 - "compilerOptions"
Cohesion: 0.17
Nodes (11): compilerOptions, declaration, module, moduleResolution, outDir, rootDir, skipLibCheck, strict (+3 more)

### Community 73 - "list_schema_files"
Cohesion: 0.29
Nodes (4): list_schema_files(), run(), list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema()

### Community 74 - "core crate"
Cohesion: 0.23
Nodes (12): Later Backlog, README Drift, cli crate, Connectivity Stubs, core crate, MigrationRenderer, MigrationStore, Planned Database Providers (+4 more)

### Community 75 - "deleteRecord"
Cohesion: 0.50
Nodes (4): deleteLikes(), deletePost(), deleteRecord(), deleteUser()

### Community 76 - "Contributing"
Cohesion: 0.17
Nodes (12): Adding a database provider, Coding agents, Contributing, Development, Prerequisites, Tests, Workspace, Client Generators (+4 more)

### Community 78 - "lib.js"
Cohesion: 0.35
Nodes (7): assetName(), cachePath(), downloadBinary(), ensureBinary(), releaseUrl(), rustTarget(), vitest

### Community 80 - "applyNestedWrites"
Cohesion: 0.36
Nodes (10): applyNestedWrites(), assertNoNestedWrites(), collectComputed(), createWithNested(), isNestedWriteBag(), readStoredLink(), rowHasNestedWrite(), splitAndResolveWriteData() (+2 more)

### Community 81 - "buildProjection"
Cohesion: 0.29
Nodes (10): appendLimitStart(), buildOrderBy(), buildProjection(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField() (+2 more)

### Community 82 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 83 - "typescript/package.json"
Cohesion: 0.10
Nodes (19): dependencies, awesome-schema, devDependencies, surrealdb, testcontainers, @types/node, typescript, vitest (+11 more)

### Community 84 - "surrealdb_migration_ledger.rs"
Cohesion: 0.20
Nodes (8): ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql(), SurrealDbMigrationLedger, with_runtime()

### Community 85 - "bug_report.md"
Cohesion: 0.33
Nodes (5): Command, Schema snippet, SurrealDB version, What happened, What you expected

### Community 86 - "feature_request.md"
Cohesion: 0.50
Nodes (3): Command or surface, Provider, What you want

### Community 89 - "MigrationOperation"
Cohesion: 0.12
Nodes (16): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+8 more)

### Community 90 - "Migration Workflow"
Cohesion: 0.47
Nodes (6): Migration Lifecycle, DropIndex Gap, migrate apply Gap, MigrationOperation, Migration Workflow, migrations crate

### Community 91 - "package.json"
Cohesion: 0.33
Nodes (5): devDependencies, awesome-schema, awesome-schema, packageManager, private

### Community 94 - "OnDeleteAction"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 95 - "./runtime"
Cohesion: 0.33
Nodes (6): exports, ./runtime, default, import, require, types

### Community 96 - "DatabaseSchema"
Cohesion: 0.20
Nodes (11): DatabaseSchema, Generator, print_generator(), ComputedBacklink, edge_name_for_table(), map_record_links(), model_name_for_table(), pair_pulled_links() (+3 more)

### Community 97 - "parse.rs"
Cohesion: 0.18
Nodes (6): load_directory_schema(), SchemaFileSource, link_target_from_type(), parse_index_fields(), ParsedFieldAttributes, validate_field_rules()

### Community 98 - "scripts"
Cohesion: 0.40
Nodes (5): scripts, build, postinstall, prepack, test

### Community 100 - "parse_field_define"
Cohesion: 0.21
Nodes (12): index_analyzer_and_hnsw_empty_values_are_ignored(), index_define_errors_too_short_and_missing_fields(), map_fields(), map_indexes(), parse_field_define(), parse_index_define(), parses_plain_index_without_unique_flag(), read_expression() (+4 more)

### Community 101 - "buildWhere"
Cohesion: 0.32
Nodes (8): buildWhere(), compileObjectWhere(), countRecords(), isScalarFilterObject(), listRelationPredicate(), nextWhereVar(), relationMode(), scalarPredicate()

### Community 102 - "btreemap"
Cohesion: 0.24
Nodes (5): introspect_async(), SurrealDbIntrospector, maps_minimal_user_fixture(), maps_minimal_user_info_json_fixture(), maps_relation_table_to_edge()

### Community 105 - "NamingConvention"
Cohesion: 0.42
Nodes (7): Edge, NamingConvention, diff_edge(), field_operations_for_edge(), field_operations_for_model(), index_operations_for_model(), relation_endpoints()

### Community 107 - "codegen-rust/tests/generator.rs"
Cohesion: 0.38
Nodes (3): maps_all_field_types_to_rust(), rejects_unimplemented_rust_provider(), schema_with_field()

### Community 109 - "devDependencies"
Cohesion: 0.50
Nodes (4): devDependencies, surrealdb, typescript, vitest

### Community 111 - "ObjectTypeField"
Cohesion: 0.60
Nodes (4): ObjectTypeDefinition, ObjectTypeField, print_object_type(), print_object_type_field()

### Community 118 - "database_config.rs"
Cohesion: 0.11
Nodes (17): connect(), execute_script_async(), execute_script_reports_connection_errors(), split_surql(), SurrealDbExecutor, builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url() (+9 more)

## Knowledge Gaps
- **342 isolated node(s):** `FIXTURE`, `FieldWriteMeta`, `GroupByAlias`, `LiveAction`, `ManyReturnResult` (+337 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 586 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `Parser`, `DatabaseConfig`, `AppContext`, `usecases.rs`, `surrealdb/mod.rs`, `introspect.rs`, `generate_code.rs`, `FieldType`, `SchemaSource`, `db.rs`, `codegen-typescript/src/lib.rs`, `DatabaseExecutor`, `arc`, `parser/src/lib.rs`, `core`, `list_schema_files`, `surrealdb_migration_ledger.rs`, `parse.rs`, `parse_field_define`, `btreemap`, `database_config.rs`?**
  _High betweenness centrality (0.077) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `FIXTURE`, `FieldWriteMeta`, `GroupByAlias` to the rest of the system?**
  _342 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `DatabaseConfig` be split into smaller, more focused modules?**
  _Cohesion score 0.10795454545454546 - nodes in this community are weakly interconnected._
- **Why does `DatabaseSchema` connect `DatabaseSchema` to `Parser`, `DatabaseConfig`, `AppContext`, `coverage_gaps.rs`, `usecases.rs`, `surrealdb/mod.rs`, `introspect.rs`, `generate_code.rs`, `migrations/src/lib.rs`, `src/print.rs`, `usecases/db_pull.rs`, `SchemaSource`, `DomainError`, `db.rs`, `codegen-typescript/src/lib.rs`, `DatabaseExecutor`, `arc`, `parser/src/lib.rs`, `core`, `parse.rs`, `parse_field_define`, `btreemap`, `NamingConvention`, `codegen-rust/tests/generator.rs`, `ObjectTypeField`, `database_config.rs`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Should `types.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09193054136874361 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `DatabaseConfig`, `coverage_gaps.rs`, `list_schema_files`, `schema_core`, `usecases.rs`, `di.rs`, `usecases/db_pull.rs`, `SchemaSource`, `DomainError`, `db.rs`, `generate_code.rs`, `DatabaseExecutor`?**
  _High betweenness centrality (0.020) - this node is a cross-community bridge._