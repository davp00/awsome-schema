# Graph Report - awsome-schema  (2026-11-20)

## Corpus Check
- 196 files · ~108,108 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .xml 3, .schema 3)

## Summary
- 2158 nodes · 4653 edges · 142 communities (85 shown, 57 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 153 edges (avg confidence: 0.89)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4646fa70`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FieldType
- coverage_gaps.rs
- Rust Best Practices
- types.rs
- codegen-typescript/tests/generator.rs
- Printer
- dispatch.rs
- cli crate
- Install and upgrade
- Automated Testing
- usecases.rs
- Token
- javascript/package.json
- surrealdb/mod.rs
- runtime.ts
- migrations/src/lib.rs
- updateManyRecords
- validation.rs
- codegen-rust/tests/generator.rs
- Milestone 1 Offline Schema Toolchain
- naming.rs
- Performance Mindset
- e2e
- introspect.rs
- ImportUsersUseCase
- Git Commit Skill
- normalize.rs
- generate_code.rs
- github_release_client.rs
- setupClient.ts
- templates.rs
- check.sh
- install-hooks.sh
- Status Check 2026-10-06
- core
- compile-client.cjs
- migrate_apply.rs
- Awesome Feature — gate checklist
- cargo clippy
- Understanding Pointers
- upgrade_cli.rs
- domain.rs
- Error Handling
- Comments versus Documentation
- buildWhere
- DatabaseExecutor
- common/mod.rs
- migrate_status.rs
- normalizeThing
- database_config.rs
- DatabaseSchema
- NamingConvention
- globalSetup.ts
- surreal.ts
- usecases/db_pull.rs
- manual-test-indexes.sh
- OnDeleteAction
- tests/db_pull.rs
- findUniqueRecord
- compilerOptions
- adapters/binary_installer.rs
- codegen-typescript/src/lib.rs
- parser/src/lib.rs
- findManyRecords
- super
- index.rs
- FileSystemPort
- compilerOptions
- surrealdb_executor.rs
- core crate
- live.test.ts
- Contributing
- lib.js
- coverage.sh
- runtime.test.ts
- schema_core
- typescript/package.json
- DatabaseConfig
- bug_report.md
- feature_request.md
- PULL_REQUEST_TEMPLATE.md
- SECURITY.md
- MigrationOperation
- Split-Core Hexagonal CLI Architecture
- package.json
- DomainError
- scripts
- Client Generators
- Commands
- AppContext
- Infrastructure Layer
- result
- UpgradeFrom
- install.sh
- MigrateCommands
- awesome-schema
- awesome-schema-codegen
- awesome-schema-codegen-rust
- awesome-schema-codegen-typescript
- awesome-schema-core
- awesome-schema-migrations
- awesome-schema-parser
- awesome-schema-renderers

## God Nodes (most connected - your core abstractions)
1. `DomainError` - 112 edges
2. `map_database_info()` - 44 edges
3. `validate_schema()` - 39 edges
4. `AppContext` - 37 edges
5. `Model` - 37 edges
6. `sample_schema()` - 35 edges
7. `FieldType` - 31 edges
8. `DatabaseSchema` - 30 edges
9. `sample_schema()` - 29 edges
10. `emit_schema_types()` - 29 edges

## Surprising Connections (you probably didn't know these)
- `cli crate` --semantically_similar_to--> `cli crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `core crate` --semantically_similar_to--> `core crate`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `Hexagonal Architecture` --semantically_similar_to--> `Split-Core Hexagonal CLI Architecture`  [INFERRED] [semantically similar]
  README.md → .agents/skills/split-core-hexagonal-cli/SKILL.md
- `Intersection SurrealQL / runtime` --references--> `encodeRecordIdString()`  [INFERRED]
  docs/roadmap/surrealdb-matrix.md → clients/javascript/src/surreal.ts
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

## Communities (142 total, 57 thin omitted)

### Community 0 - "FieldType"
Cohesion: 0.12
Nodes (16): FieldType, Array, Bool, Custom, Datetime, Float, Int, Object (+8 more)

### Community 1 - "coverage_gaps.rs"
Cohesion: 0.05
Nodes (27): ensure_can_write(), ensure_can_write_ok_for_empty_tables_dir_and_empty_file(), fs_result(), NoopPull, run_pull(), run_push(), write_single_schema(), write_split_schema() (+19 more)

### Community 2 - "Rust Best Practices"
Cohesion: 0.20
Nodes (7): Generics and Dispatch, PhantomData, Swift Type State, Type State Pattern, TypeScript Type State, Apollo Rust Best Practices Handbook, Rust Best Practices

### Community 3 - "types.rs"
Cohesion: 0.08
Nodes (86): client_key(), emit_crud_helpers(), emit_fluent_client(), emit_fluent_edge_delegate(), emit_fluent_model_delegate(), emit_query_helpers(), emit_select_helpers(), emit_session_live() (+78 more)

### Community 4 - "codegen-typescript/tests/generator.rs"
Cohesion: 0.08
Nodes (28): computed_link_and_relation_omitted_on_record_present_on_selected(), emits_as_record_id_and_crud_query_helpers(), emits_count_and_groupby(), emits_create_update_inputs_without_computed_fields(), emits_dual_shapes_for_models_and_edges(), emits_find_unique_hybrid_where(), emits_fluent_create_client(), emits_get_payload_and_select_types() (+20 more)

### Community 5 - "Printer"
Cohesion: 0.17
Nodes (7): run_apply(), run_create(), run_dev(), run_rollback(), run_status(), run(), Printer

### Community 6 - "dispatch.rs"
Cohesion: 0.10
Nodes (19): dispatch_db_pull_refuses_overwrite_without_force(), dispatch_db_pull_reports_connection_error_without_server(), dispatch_db_push_returns_connection_error_without_server(), dispatch_format_write_command(), dispatch_generate_targets(), dispatch_generate_typescript_writes_output(), dispatch_init_command(), dispatch_init_when_schema_already_exists() (+11 more)

### Community 7 - "cli crate"
Cohesion: 0.17
Nodes (12): Incremental Strangle Migration, Migration from Monolithic main.rs, Service to CLI Migration, AppContext, ImportUsersUseCase, mockall, UserRepository, AppContext (+4 more)

### Community 8 - "Install and upgrade"
Cohesion: 0.06
Nodes (31): Awesome Schema DSL (SurrealDB), Datasource and generator, Field assignments, Models, links, edges, Push vs pull vs migrate, Split schema directories, Common commands, Global flags (+23 more)

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
Cohesion: 0.07
Nodes (29): bin, awesome-schema, description, devDependencies, surrealdb, typescript, vitest, engines (+21 more)

### Community 14 - "surrealdb/mod.rs"
Cohesion: 0.12
Nodes (30): NamingContext, default_renderer_and_schemaless_edge_permissions(), preserves_field_name_when_fields_naming_not_set(), render_computed_link(), render_define_field(), render_define_index(), render_define_relation_table(), render_define_table() (+22 more)

### Community 15 - "runtime.ts"
Cohesion: 0.10
Nodes (23): applyNestedWrites(), bindNested(), flushOps(), GroupByAlias, insertStoredCreates(), isRecordIdOnlyFilter(), LiveAction, ManyReturnResult (+15 more)

### Community 16 - "migrations/src/lib.rs"
Cohesion: 0.22
Nodes (26): creates_indexes_when_model_is_new(), creates_initial_migration_from_empty_snapshot(), creates_permission_on_new_edge(), creates_relation_table_for_new_edge(), detects_added_field(), detects_altered_field_and_permissions(), detects_altered_table_mode(), detects_cleared_permissions() (+18 more)

### Community 17 - "updateManyRecords"
Cohesion: 0.25
Nodes (15): asRows(), assertNonEmptyWhere(), assertReturnSelectExclusive(), buildProjection(), countMatching(), createManyVia(), deleteManyRecords(), deleteOneRecord() (+7 more)

### Community 18 - "validation.rs"
Cohesion: 0.14
Nodes (40): accepts_index_on_known_field(), accepts_relation_field_matching_edge_map_attribute(), accepts_relation_field_matching_edge_table_name(), accepts_relation_field_naming_existing_edge(), accepts_single_sided_named_link(), bare_field(), id_field(), is_reserved_object_field_name() (+32 more)

### Community 19 - "codegen-rust/tests/generator.rs"
Cohesion: 0.38
Nodes (3): maps_all_field_types_to_rust(), rejects_unimplemented_rust_provider(), schema_with_field()

### Community 20 - "Milestone 1 Offline Schema Toolchain"
Cohesion: 0.15
Nodes (18): Milestone 1 Offline Schema Toolchain, Post-Plan Work on main, Roadmap Notes, COMPUTED Clause, DEFINE FIELD Compatibility, FLEXIBLE Semantics Change, id Field Enforcement, SurrealDB Rust SDK Upgrade (+10 more)

### Community 21 - "naming.rs"
Cohesion: 0.09
Nodes (18): map_attribute_overrides_convention(), map_field_path(), mapped_name(), maps_dotted_field_paths_per_segment(), naming_context_resolves_table_and_field_names(), NamingCase, CamelCase, KebabCase (+10 more)

### Community 22 - "Performance Mindset"
Cohesion: 0.18
Nodes (7): Coding Styles and Idioms, rustfmt, cargo bench, cargo flamegraph, Performance Mindset, samply, smallvec

### Community 24 - "introspect.rs"
Cohesion: 0.07
Nodes (65): computed_without_backlink_and_weird_body_are_not_backlinks(), ComputedBacklink, default_relation_field_name(), edge_name_for_table(), ensure_model_id_field(), ensure_relation_nav_field(), field_define_errors_propagate_through_map_fields(), id_field_non_record_keeps_scalar_type() (+57 more)

### Community 25 - "ImportUsersUseCase"
Cohesion: 0.14
Nodes (20): CLI Architecture Examples, FileSystemPort, ImportUsersCommand, ImportUsersUseCase, JsonFormatter, OutputWriter, User, UserDatabaseRepository (+12 more)

### Community 26 - "Git Commit Skill"
Cohesion: 0.48
Nodes (3): Breaking Change, Conventional Commits, Git Commit Skill

### Community 27 - "normalize.rs"
Cohesion: 0.06
Nodes (66): Field, Model, LinkStorage, Computed, Stored, bare_field(), empty_base_schema(), ensure_on_delete_default() (+58 more)

### Community 28 - "generate_code.rs"
Cohesion: 0.10
Nodes (24): FormatSchemaInput, FormatSchemaOutput, FormatSchemaUseCase, normalize_whitespace(), CLIENT_PACKAGE_JSON, client_package_json_path(), CodeGeneratorPort, DirAwareFs (+16 more)

### Community 29 - "github_release_client.rs"
Cohesion: 0.12
Nodes (4): GithubReleaseClient, http_get(), http_get_bytes(), RELEASE_REPO

### Community 30 - "setupClient.ts"
Cohesion: 0.16
Nodes (5): connectSurreal(), openGeneratedClient(), readE2eEnv(), recordIdString(), deleteByEmail()

### Community 36 - "Status Check 2026-10-06"
Cohesion: 0.15
Nodes (19): Real Graph Edges, Index Kinds, Migration Lifecycle, SchemaDiffer, db push, DropIndex Gap, edge Block Gap, Index Flag Gap (+11 more)

### Community 37 - "core"
Cohesion: 0.24
Nodes (3): map_field_type(), RustGenerator, CodeGenerator

### Community 38 - "compile-client.cjs"
Cohesion: 0.18
Nodes (12): bindings(), compileClientFile(), compileGeneratedClient(), { dirname, join, resolve, sep }, esmToCjs(), { existsSync, readFileSync, writeFileSync }, findGeneratedClient(), { stripTypeScriptTypes } (+4 more)

### Community 39 - "migrate_apply.rs"
Cohesion: 0.39
Nodes (4): checksum_of(), MigrateApplyInput, MigrateApplyOutput, MigrateApplyUseCase

### Community 40 - "Awesome Feature — gate checklist"
Cohesion: 0.10
Nodes (18): Architecture, Awesome Feature — gate checklist, Docs / memory / git, Generated-client e2e, Plan, Rust e2e, Unit / coverage, Awesome Feature (+10 more)

### Community 41 - "cargo clippy"
Cohesion: 0.44
Nodes (7): cargo clippy, Clippy and Linting Discipline, clone_on_copy, large_enum_variant, manual_ok_or, needless_collect, redundant_clone

### Community 42 - "Understanding Pointers"
Cohesion: 0.40
Nodes (5): Arc, Box, Rc, Rust Atomics and Locks, Understanding Pointers

### Community 43 - "upgrade_cli.rs"
Cohesion: 0.15
Nodes (18): already_up_to_date_skips_download(), check_reports_available_update(), crates_io_source_invokes_installer(), CratesInstaller, empty_target_is_unsupported(), FakeRelease, github_upgrade_replaces_binary(), normalize_tag() (+10 more)

### Community 44 - "domain.rs"
Cohesion: 0.12
Nodes (6): MigrationPlan, MigrationRenderer, MigrateDevInput, MigrateDevOutput, MigrateDevUseCase, SchemaDiffPort

### Community 45 - "Error Handling"
Cohesion: 0.44
Nodes (3): anyhow, Error Handling, thiserror

### Community 46 - "Comments versus Documentation"
Cohesion: 0.42
Nodes (5): Architectural Decision Record, Comments versus Documentation, missing_docs, non_exhaustive, rustdoc

### Community 47 - "buildWhere"
Cohesion: 0.20
Nodes (14): appendLimitStart(), assertAggregateFilter(), buildGroupByOrderBy(), buildGroupHaving(), buildWhere(), compileObjectWhere(), countRecords(), groupByRecords() (+6 more)

### Community 48 - "DatabaseExecutor"
Cohesion: 0.18
Nodes (5): DatabaseExecutor, DbPushInput, DbPushOutput, DbPushUseCase, ensure_fulltext_analyzers_exist()

### Community 49 - "common/mod.rs"
Cohesion: 0.06
Nodes (25): MINIMAL_SCHEMA, UNREACHABLE_SCHEMA, connect(), docker_unavailable(), edge_schema(), field_define(), MINIMAL_SCHEMA, record_ref_schema() (+17 more)

### Community 50 - "migrate_status.rs"
Cohesion: 0.22
Nodes (7): MigrateStatusInput, MigrateStatusOutput, MigrateStatusUseCase, MigrationApplyState, Applied, Pending, MigrationStatusRow

### Community 51 - "normalizeThing"
Cohesion: 0.18
Nodes (23): asRecord(), collectComputed(), collectEdge(), deleteRecord(), edgeConnectRows(), edgeContent(), edgeFarData(), edgeParentField() (+15 more)

### Community 54 - "database_config.rs"
Cohesion: 0.15
Nodes (13): connect(), builds_config_from_literal_url(), normalize_ws_endpoint(), normalizes_http_url(), preserves_wss_endpoint(), rejects_missing_env_var(), rejects_missing_url(), rejects_unsupported_provider() (+5 more)

### Community 55 - "DatabaseSchema"
Cohesion: 0.09
Nodes (11): MigrationStoreAdapter, DatabaseSchema, MigrationStore, SchemaRenderer, SchemaSource, MigrateCreateInput, MigrateCreateOutput, MigrateCreateUseCase (+3 more)

### Community 56 - "NamingConvention"
Cohesion: 0.31
Nodes (7): NamingConvention, diff_edge(), diff_model(), field_operations_for_edge(), field_operations_for_model(), index_operations_for_model(), relation_endpoints()

### Community 57 - "globalSetup.ts"
Cohesion: 0.22
Nodes (12): formatError(), globalSetup(), runCli(), writeEnv(), E2eEnv, envPath, findWorkspaceRoot(), fixtureSchemaPath (+4 more)

### Community 58 - "surreal.ts"
Cohesion: 0.12
Nodes (13): LiveHandle, SurrealLike, SurrealOpsLike, SurrealTransactionLike, ADAPTED, asSurrealLike(), asSurrealTransaction(), bindOps() (+5 more)

### Community 59 - "usecases/db_pull.rs"
Cohesion: 0.38
Nodes (4): DbPullInput, DbPullOutput, DbPullUseCase, edges_lost()

### Community 60 - "manual-test-indexes.sh"
Cohesion: 0.70
Nodes (4): cleanup(), compose(), manual-test-indexes.sh script, surreal_sql()

### Community 61 - "OnDeleteAction"
Cohesion: 0.25
Nodes (5): OnDeleteAction, Cascade, Ignore, Reject, Unset

### Community 63 - "findUniqueRecord"
Cohesion: 0.40
Nodes (6): findUniqueRecord(), firstRow(), selectRecord(), selectRecordRelated(), uniqueWhereId(), updateRecord()

### Community 64 - "compilerOptions"
Cohesion: 0.15
Nodes (12): compilerOptions, esModuleInterop, module, moduleResolution, noEmit, resolveJsonModule, rootDir, skipLibCheck (+4 more)

### Community 65 - "adapters/binary_installer.rs"
Cohesion: 0.08
Nodes (10): detect_install_kind(), FsBinaryInstaller, replace_atomic(), replace_atomic_writes_executable(), BinaryInstaller, InstallKind, NpmManaged, Standalone (+2 more)

### Community 66 - "codegen-typescript/src/lib.rs"
Cohesion: 0.20
Nodes (5): SURREAL, typescript_provider(), TypeScriptGenerator, TypeScriptProvider, SurrealTypeScriptProvider

### Community 67 - "parser/src/lib.rs"
Cohesion: 0.21
Nodes (14): EXAMPLE, parse(), parses_edge_block(), parses_example_schema(), parses_example_schema_naming(), parses_field_default_value(), parses_field_value_and_updated_attributes(), parses_minimal_datasource() (+6 more)

### Community 68 - "findManyRecords"
Cohesion: 0.36
Nodes (8): buildOrderBy(), collectOrderByCountProjections(), findManyRecords(), nestedLimitClause(), orderByCountAlias(), projectField(), relationCountExpr(), stripOrderByCountFields()

### Community 69 - "super"
Cohesion: 0.16
Nodes (6): RelationEndpoints, ObjectTypeDefinition, ObjectTypeField, Relation, Generator, merge_pulled_schema()

### Community 70 - "index.rs"
Cohesion: 0.14
Nodes (6): Index, VectorDist, Cosine, Euclidean, Manhattan, Model

### Community 71 - "FileSystemPort"
Cohesion: 0.11
Nodes (9): list_schema_files(), load_directory_schema(), SchemaFileSource, list_schema_files_directory_without_tables_subdir(), schema_file_source_directory_and_skip_non_schema(), FileSystemPort, InitProjectInput, InitProjectOutput (+1 more)

### Community 72 - "compilerOptions"
Cohesion: 0.17
Nodes (11): compilerOptions, declaration, module, moduleResolution, outDir, rootDir, skipLibCheck, strict (+3 more)

### Community 73 - "surrealdb_executor.rs"
Cohesion: 0.17
Nodes (8): execute_script_async(), execute_script_reports_connection_errors(), list_analyzers_async(), split_surql(), SurrealDbExecutor, MigrateRollbackInput, MigrateRollbackOutput, MigrateRollbackUseCase

### Community 74 - "core crate"
Cohesion: 0.23
Nodes (11): db pull, Later Backlog, cli crate, Connectivity Stubs, core crate, MigrationStore, Planned Database Providers, renderers crate (+3 more)

### Community 75 - "live.test.ts"
Cohesion: 0.33
Nodes (3): deleteByEmail(), LiveAction, LiveHandle

### Community 76 - "Contributing"
Cohesion: 0.11
Nodes (15): awesome-schema, Install → generate → e2e, Supported SurrealDB matrix, Adding a database provider, Coding agents, Contributing, Development, Prerequisites (+7 more)

### Community 78 - "lib.js"
Cohesion: 0.18
Nodes (12): child, require, { version }, assetName(), cachePath(), downloadBinary(), ensureBinary(), releaseUrl() (+4 more)

### Community 80 - "runtime.test.ts"
Cohesion: 0.13
Nodes (17): asRecordId(), assertNoNestedWrites(), bindSchemaMeta(), createWithNested(), FieldSelectMeta, FieldWriteMeta, isNestedWriteBag(), isTransactionClient() (+9 more)

### Community 83 - "typescript/package.json"
Cohesion: 0.10
Nodes (20): dependencies, awesome-schema, devDependencies, surrealdb, testcontainers, @types/node, typescript, vitest (+12 more)

### Community 84 - "DatabaseConfig"
Cohesion: 0.12
Nodes (14): introspect_async(), SurrealDbIntrospector, ensure_schema_reports_connection_errors(), ENSURE_SCHEMA_SCRIPT, escape_string(), LEDGER_TABLE, parse_ledger_row(), split_surql() (+6 more)

### Community 85 - "bug_report.md"
Cohesion: 0.33
Nodes (5): Command, Schema snippet, SurrealDB version, What happened, What you expected

### Community 86 - "feature_request.md"
Cohesion: 0.50
Nodes (3): Command or surface, Provider, What you want

### Community 89 - "MigrationOperation"
Cohesion: 0.10
Nodes (20): MigrationOperation, AlterField, AlterTable, CreateEvent, CreateField, CreateFunction, CreateIndex, CreatePermission (+12 more)

### Community 90 - "Split-Core Hexagonal CLI Architecture"
Cohesion: 0.15
Nodes (15): CLI Architecture Checklist, resolveExitCode, CLI Architecture Reference, Error Catalog, anyhow, awesome-schema binary, Cargo Workspace Layout, DomainError (+7 more)

### Community 91 - "package.json"
Cohesion: 0.33
Nodes (5): devDependencies, awesome-schema, awesome-schema, packageManager, private

### Community 97 - "DomainError"
Cohesion: 0.07
Nodes (18): FsAdapter, DomainError, CodegenError, DatabaseError, DownloadFailed, MigrationError, NotImplemented, NpmManagedInstall (+10 more)

### Community 98 - "scripts"
Cohesion: 0.40
Nodes (5): scripts, build, postinstall, prepack, test

### Community 101 - "Client Generators"
Cohesion: 0.50
Nodes (5): Client Generators, Codegen Stub Gap, codegen crate, codegen-rust, codegen-typescript

### Community 105 - "Commands"
Cohesion: 0.12
Nodes (17): Cli, Commands, Db, Format, Generate, Init, Migrate, Upgrade (+9 more)

### Community 118 - "AppContext"
Cohesion: 0.12
Nodes (3): AppContext, build_context(), build_context_with_introspector()

### Community 132 - "Infrastructure Layer"
Cohesion: 0.29
Nodes (3): AppLogger, Orchestrator, Shared Capability Package

### Community 136 - "UpgradeFrom"
Cohesion: 0.47
Nodes (4): UpgradeFrom, CratesIo, Github, run()

### Community 138 - "install.sh"
Cohesion: 0.38
Nodes (9): asset_name(), extract_tags(), gh_get(), is_usable_tag(), latest_stable_tag(), main(), normalize_version(), rust_target() (+1 more)

### Community 139 - "MigrateCommands"
Cohesion: 0.33
Nodes (6): MigrateCommands, Apply, Create, Dev, Rollback, Status

## Knowledge Gaps
- **313 isolated node(s):** `Summary`, `Test plan`, `Prerequisites`, `Workspace`, `Coding agents` (+308 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 632 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **57 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DomainError` connect `DomainError` to `adapters/binary_installer.rs`, `codegen-typescript/src/lib.rs`, `parser/src/lib.rs`, `core`, `FileSystemPort`, `migrate_apply.rs`, `upgrade_cli.rs`, `domain.rs`, `DatabaseConfig`, `database_config.rs`, `DatabaseSchema`, `AppContext`, `github_release_client.rs`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `map_database_info()` (e.g. with `introspect_async()` and `maps_minimal_user_fixture()`) actually correct?**
  _`map_database_info()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Summary`, `Test plan`, `Prerequisites` to the rest of the system?**
  _313 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FieldType` be split into smaller, more focused modules?**
  _Cohesion score 0.12343358395989974 - nodes in this community are weakly interconnected._
- **Why does `AppContext` connect `AppContext` to `coverage_gaps.rs`, `Printer`, `result`, `UpgradeFrom`, `Commands`, `upgrade_cli.rs`, `schema_core`, `generate_code.rs`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Should `coverage_gaps.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05352112676056338 - nodes in this community are weakly interconnected._
- **Should `types.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08242908814011041 - nodes in this community are weakly interconnected._