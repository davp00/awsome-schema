# Graph Report - awsome-schema  (2026-11-20)

## Corpus Check
- 185 files · ~111,985 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 15 file(s) not represented in the graph (top: (none) 5, .xml 3, .schema 2)

## Summary
- 2170 nodes · 4692 edges · 128 communities (83 shown, 45 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 153 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9eef240f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FieldType
- coverage_gaps.rs
- Rust Best Practices
- types.rs
- codegen-typescript/tests/generator.rs
- AppContext
- dispatch.rs
- cli crate
- DatabaseExecutor
- Automated Testing
- usecases.rs
- Token
- javascript/package.json
- surrealdb/mod.rs
- runtime.ts
- index.ts
- queryRows
- validation.rs
- codegen-rust/tests/generator.rs
- Migration Workflow
- migrations/src/lib.rs
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
- core
- compile-client.cjs
- buildClientDelegates
- Awesome Feature — gate checklist
- cargo clippy
- Understanding Pointers
- updateWithNested
- OnDeleteAction
- Error Handling
- Comments versus Documentation
- normalizeThing
- DomainError
- common/mod.rs
- migrate_status.rs
- normalizeThing
- database_config.rs
- SchemaSource
- runtime.test.ts
- globalSetup.ts
- surreal.ts
- src/print.rs
- manual-test-indexes.sh
- lib.js
- tests/db_pull.rs
- domain.rs
- compilerOptions
- buildWhere
- codegen-typescript/src/lib.rs
- parser/src/lib.rs
- findManyRecords
- btreemap
- index.rs
- FileSystemPort
- compilerOptions
- migrate_rollback.rs
- core crate
- deleteRecord
- Contributing
- awesome-schema.js
- coverage.sh
- splitAndResolveWriteData
- usecases/db_pull.rs
- Infrastructure Layer
- typescript/package.json
- DatabaseConfig
- bug_report.md
- feature_request.md
- PULL_REQUEST_TEMPLATE.md
- SECURITY.md
- MigrationOperation
- Split-Core Hexagonal CLI Architecture
- package.json
- ./runtime
- DomainError
- scripts
- findUniqueRecord
- model.rs
- surrealdb_connection.rs

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 78 edges
2. `map_database_info()` - 43 edges
3. `validate_schema()` - 39 edges
4. `Model` - 37 edges
5. `sample_schema()` - 35 edges
6. `AppContext` - 33 edges
7. `FieldType` - 31 edges
8. `DatabaseSchema` - 31 edges
9. `emit_schema_types()` - 29 edges
10. `sample_schema()` - 29 edges

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

## Communities (128 total, 45 thin omitted)

### Community 0 - "FieldType"
Cohesion: 0.12
Nodes (16): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+8 more)

### Community 1 - "coverage_gaps.rs"
Cohesion: 0.10
Nodes (14): test_context_with_introspector(), EmptyLedger, init_reports_schema_created(), migrate_apply_and_rollback_success_messages(), migrate_status_empty_and_rollback_zero_steps(), migrate_status_lists_applied_and_pending_with_snapshot(), MINIMAL_SCHEMA, OkExecutor (+6 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "types.rs"
Cohesion: 0.08
Nodes (86): client_key(), emit_crud_helpers(), emit_fluent_client(), emit_fluent_edge_delegate(), emit_fluent_model_delegate(), emit_query_helpers(), emit_select_helpers(), emit_session_live() (+78 more)

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.08
Nodes (28): computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby(), emits_create_update_inputs_without_computed_fields(), emits_dual_shapes_for_models_and_edges(), emits_find_unique_hybrid_where(), emits_fluent_create_client(), emits_get_payload_and_select_types() (+20 more)

### Community 5 - "AppContext"
Cohesion: 0.08
Nodes (17): list_schema_files(), run(), run(), run(), run_apply(), run_create(), run_dev(), run_rollback() (+9 more)

### Community 6 - "dispatch.rs"
Cohesion: 0.05
Nodes (38): Cli, Commands, Db, Format, Generate, Init, Migrate, Validate (+30 more)

### Community 7 - "cli crate"
Cohesion: 0.17
Nodes (12): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, ImportUsersUseCase, mockall, UserRepository, AppContext (+4 more)

### Community 8 - "DatabaseExecutor"
Cohesion: 0.13
Nodes (10): execute_script_async(), execute_script_reports_connection_errors(), list_analyzers_async(), split_surql(), SurrealDbExecutor, DatabaseExecutor, DbPushInput, DbPushOutput (+2 more)

### Community 10 - "Automated Testing"
Cohesion: 0.11
Nodes (12): assert and assert_eq, Automated Testing, cargo insta, cargo nextest, pretty_assertions, rstest, testcontainers, Criterion (+4 more)

### Community 11 - "usecases.rs"
Cohesion: 0.06
Nodes (42): CountingRenderer, db_push_fails_when_fulltext_analyzer_missing(), db_push_refuses_unknown_provider_before_render(), db_push_renders_and_applies_schema(), db_push_succeeds_when_fulltext_analyzer_present(), EmptyDiff, format_schema_multiple_files_without_write_back(), format_schema_trims_trailing_blank_lines() (+34 more)

### Community 12 - "Token"
Cohesion: 0.11
Nodes (25): Lexer, Lexer<'a>, rejects_unexpected_character(), rejects_unterminated_string(), Token, At, Comma, Dot (+17 more)

### Community 13 - "javascript/package.json"
Cohesion: 0.08
Nodes (23): bin, awesome-schema, description, devDependencies, surrealdb, typescript, vitest, engines (+15 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.12
Nodes (30): NamingContext, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table() (+22 more)

### Community 15 - "runtime.ts"
Cohesion: 0.09
Nodes (36): appendLimitStart(), assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildOrderBy(), buildWhere(), collectOrderByCountProjections(), compileObjectWhere() (+28 more)

### Community 16 - "index.ts"
Cohesion: 0.02
Nodes (84): AggregateFilter, ArrayFilter, BooleanFilter, DateFilter, FieldSelectMeta, FieldWriteMeta, GroupByAlias, IdFilter (+76 more)

### Community 17 - "queryRows"
Cohesion: 0.30
Nodes (12): asRows(), assertNonEmptyWhere(), assertReturnSelectExclusive(), buildProjection(), countMatching(), deleteManyRecords(), deleteOneRecord(), projectRow() (+4 more)

### Community 18 - "validation.rs"
Cohesion: 0.14
Nodes (40): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), is_reserved_object_field_name() (+32 more)

### Community 19 - "codegen-rust/tests/generator.rs"
Cohesion: 0.38
Nodes (3): maps_all_field_types_to_rust(), rejects_unimplemented_rust_provider(), schema_with_field()

### Community 20 - "Migration Workflow"
Cohesion: 0.17
Nodes (16): Post-Plan Work on main, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, Awesome Schema, Awesome Schema DSL, Field Assignments (+8 more)

### Community 21 - "migrations/src/lib.rs"
Cohesion: 0.06
Nodes (51): map_attribute_overrides_convention(), map_field_path(), mapped_name(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingCase, CamelCase, KebabCase (+43 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 23 - "cli"
Cohesion: 0.39
Nodes (9): cli, codegen, codegen-rust, codegen-typescript, core, e2e, migrations, parser (+1 more)

### Community 24 - "introspect.rs"
Cohesion: 0.08
Nodes (64): computed_without_backlink_and_weird_body_are_not_backlinks(), ComputedBacklink, default_relation_field_name(), edge_name_for_table(), ensure_model_id_field(), ensure_relation_nav_field(), field_define_errors_propagate_through_map_fields(), id_field_non_record_keeps_scalar_type() (+56 more)

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
Cohesion: 0.10
Nodes (24): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace(), CLIENT_PACKAGE_JSON, client_package_json_path(), CodeGeneratorPort, DirAwareFs (+16 more)

### Community 30 - "setupClient.ts"
Cohesion: 0.13
Nodes (8): connectSurreal(), openGeneratedClient(), readE2eEnv(), recordIdString(), deleteByEmail(), LiveAction, LiveHandle, deleteByEmail()

### Community 36 - "Status Check 2026-10-06"
Cohesion: 0.11
Nodes (26): Client Generators, Real Graph Edges, Index Kinds, Later Backlog, Migration Lifecycle, SchemaDiffer, Milestone 1 Offline Schema Toolchain, Roadmap Notes (+18 more)

### Community 37 - "core"
Cohesion: 0.24
Nodes (3): map_field_type(), RustGenerator, CodeGenerator

### Community 38 - "compile-client.cjs"
Cohesion: 0.19
Nodes (12): bindings(), compileClientFile(), compileGeneratedClient(), { dirname, join, resolve, sep }, esmToCjs(), { existsSync, readFileSync, writeFileSync }, findGeneratedClient(), { stripTypeScriptTypes } (+4 more)

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

### Community 44 - "OnDeleteAction"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "normalizeThing"
Cohesion: 0.13
Nodes (20): collectEdge(), createLikes(), edgeContent(), edgeFarData(), findUniqueRecord(), firstRow(), normalizeThing(), relateEdge() (+12 more)

### Community 48 - "DomainError"
Cohesion: 0.17
Nodes (12): resolveExitCode, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError, resolve_exit_code, thiserror (+4 more)

### Community 49 - "common/mod.rs"
Cohesion: 0.06
Nodes (23): MINIMAL_SCHEMA, UNREACHABLE_SCHEMA, connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA, record_ref_schema() (+15 more)

### Community 50 - "migrate_status.rs"
Cohesion: 0.22
Nodes (7): MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 51 - "normalizeThing"
Cohesion: 0.18
Nodes (23): asRecord(), collectComputed(), collectEdge(), deleteRecord(), edgeConnectRows(), edgeContent(), edgeFarData(), edgeParentField() (+15 more)

### Community 54 - "database_config.rs"
Cohesion: 0.27
Nodes (10): builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), preserves_wss_endpoint(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider(), resolve_datasource_value() (+2 more)

### Community 55 - "SchemaSource"
Cohesion: 0.15
Nodes (7): SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase, ValidateSchemaInput, ValidateSchemaOutput, ValidateSchemaUseCase

### Community 56 - "runtime.test.ts"
Cohesion: 0.20
Nodes (7): asRecordId(), bindSchemaMeta(), FieldSelectMeta, FieldWriteMeta, parseRecordId(), recordId, selectMeta

### Community 57 - "globalSetup.ts"
Cohesion: 0.22
Nodes (12): formatError(), globalSetup(), runCli(), writeEnv(), E2eEnv, envPath, findWorkspaceRoot(), fixtureSchemaPath (+4 more)

### Community 58 - "surreal.ts"
Cohesion: 0.13
Nodes (11): LiveHandle, SurrealLike, SurrealOpsLike, SurrealTransactionLike, ADAPTED, asSurrealLike(), asSurrealTransaction(), bindOps() (+3 more)

### Community 59 - "src/print.rs"
Cohesion: 0.06
Nodes (38): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), NoopPull, run_pull(), run_push(), write_single_schema(), write_split_schema() (+30 more)

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "lib.js"
Cohesion: 0.47
Nodes (6): assetName(), cachePath(), downloadBinary(), ensureBinary(), releaseUrl(), rustTarget()

### Community 63 - "domain.rs"
Cohesion: 0.15
Nodes (4): checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "buildWhere"
Cohesion: 0.18
Nodes (12): assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildWhere(), countRecords(), groupByRecords(), isScalarFilterObject(), listRelationPredicate() (+4 more)

### Community 66 - "codegen-typescript/src/lib.rs"
Cohesion: 0.20
Nodes (5): SURREAL, typescript_provider(), TypeScriptGenerator, TypeScriptProvider, SurrealTypeScriptProvider

### Community 67 - "parser/src/lib.rs"
Cohesion: 0.21
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 68 - "findManyRecords"
Cohesion: 0.31
Nodes (9): appendLimitStart(), buildOrderBy(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField(), relationCountExpr() (+1 more)

### Community 69 - "btreemap"
Cohesion: 0.15
Nodes (6): RelationEndpoints, ObjectTypeDefinition, ObjectTypeField, Relation, Generator, merge_pulled_schema()

### Community 70 - "index.rs"
Cohesion: 0.17
Nodes (5): Index, VectorDist, Cosine, Euclidean, Manhattan

### Community 71 - "FileSystemPort"
Cohesion: 0.09
Nodes (13): load_directory_schema(), SchemaFileSource, require_surreal_provider(), MigrationPlan, FileSystemPort, MigrationRenderer, InitProjectInput, InitProjectOutput (+5 more)

### Community 72 - "compilerOptions"
Cohesion: 0.17
Nodes (11): compilerOptions, declaration, module, moduleResolution, outDir, rootDir, skipLibCheck, strict (+3 more)

### Community 73 - "migrate_rollback.rs"
Cohesion: 0.26
Nodes (3): MigrateRollbackInput, MigrateRollbackOutput, MigrateRollbackUseCase

### Community 74 - "core crate"
Cohesion: 0.27
Nodes (10): cli crate, core crate, MigrationOperation, MigrationRenderer, MigrationStore, Planned Database Providers, renderers crate, SchemaIntrospector (+2 more)

### Community 75 - "deleteRecord"
Cohesion: 0.50
Nodes (4): deleteLikes(), deletePost(), deleteRecord(), deleteUser()

### Community 76 - "Contributing"
Cohesion: 0.17
Nodes (10): awesome-schema, Install → generate → e2e, Supported SurrealDB matrix, Adding a database provider, Coding agents, Contributing, Development, Prerequisites (+2 more)

### Community 78 - "awesome-schema.js"
Cohesion: 0.20
Nodes (6): child, require, { version }, require, { version }, version

### Community 80 - "splitAndResolveWriteData"
Cohesion: 0.14
Nodes (20): applyNestedWrites(), assertNoNestedWrites(), bindNested(), createManyVia(), createWithNested(), flushOps(), insertManyRows(), insertStoredCreates() (+12 more)

### Community 81 - "usecases/db_pull.rs"
Cohesion: 0.39
Nodes (3): DbPullInput, DbPullOutput, DbPullUseCase

### Community 82 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 83 - "typescript/package.json"
Cohesion: 0.10
Nodes (20): dependencies, awesome-schema, devDependencies, surrealdb, testcontainers, @types/node, typescript, vitest (+12 more)

### Community 84 - "DatabaseConfig"
Cohesion: 0.16
Nodes (11): ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql(), SurrealDbMigrationLedger, with_runtime() (+3 more)

### Community 85 - "bug_report.md"
Cohesion: 0.33
Nodes (5): Command, Schema snippet, SurrealDB version, What happened, What you expected

### Community 86 - "feature_request.md"
Cohesion: 0.50
Nodes (3): Command or surface, Provider, What you want

### Community 89 - "MigrationOperation"
Cohesion: 0.12
Nodes (16): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+8 more)

### Community 90 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.52
Nodes (4): CLI Architecture Checklist, CLI Architecture Reference, Rust CLI Examples, Rust Implementation Guide

### Community 91 - "package.json"
Cohesion: 0.33
Nodes (5): devDependencies, awesome-schema, awesome-schema, packageManager, private

### Community 94 - "./runtime"
Cohesion: 0.33
Nodes (6): exports, ./runtime, default, import, require, types

### Community 97 - "DomainError"
Cohesion: 0.07
Nodes (20): FsAdapter, introspect_async(), SurrealDbIntrospector, MigrationStoreAdapter, DatabaseSchema, DomainError, CodegenError, DatabaseError (+12 more)

### Community 98 - "scripts"
Cohesion: 0.40
Nodes (5): scripts, build, postinstall, prepack, test

### Community 101 - "findUniqueRecord"
Cohesion: 0.40
Nodes (6): findUniqueRecord(), firstRow(), selectRecord(), selectRecordRelated(), uniqueWhereId(), updateRecord()

### Community 105 - "model.rs"
Cohesion: 0.47
Nodes (5): Edge, Model, TableMode, Schemafull, Schemaless

## Knowledge Gaps
- **348 isolated node(s):** `Supported SurrealDB matrix`, `Install → generate → e2e`, `name`, `description`, `license` (+343 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 650 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **45 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `codegen-typescript/src/lib.rs`, `parser/src/lib.rs`, `core`, `AppContext`, `FileSystemPort`, `DatabaseConfig`, `SchemaSource`, `surrealdb_connection.rs`, `database_config.rs`, `domain.rs`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Supported SurrealDB matrix`, `Install → generate → e2e`, `name` to the rest of the system?**
  _348 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FieldType` be split into smaller, more focused modules?**
  _Cohesion score 0.12343358395989974 - nodes in this community are weakly interconnected._
- **Why does `FieldType` connect `FieldType` to `btreemap`, `core`, `src/print.rs`, `validation.rs`, `codegen-rust/tests/generator.rs`, `migrations/src/lib.rs`, `normalize.rs`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Should `coverage_gaps.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09871794871794871 - nodes in this community are weakly interconnected._
- **Should `types.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08242908814011041 - nodes in this community are weakly interconnected._