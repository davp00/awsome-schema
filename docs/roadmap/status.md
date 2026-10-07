# Status

Checked on 2026-10-07 against `main` (TypeScript client e2e).

Check so far:

- Read the CLI, domain, parser, differ, SurrealDB renderer, codegen, and `db` adapters.
- `cargo test --workspace --exclude e2e` — unit/integration suite.
- `cargo test -p e2e --locked` — topic binaries against SurrealDB **v3.3.0** via Docker/testcontainers.
- `cd e2e/typescript && npm test` — Vitest generated-client CRUD + nested select e2e (hybrid URL / testcontainers); CI via `.github/workflows/typescript-e2e.yml`.

Legend: **done** means the behavior exists and is covered by unit or CLI tests. **partial** means some of the path works and a specific gap remains. **not started** means the command or type exists but does not do the job.

## Done

| Area | What works | Where it was checked |
|---|---|---|
| Workspace layout | `cli`, `core`, `parser`, `migrations`, `renderers`, `codegen`, `codegen-rust`, `codegen-typescript`, `e2e` | `Cargo.toml` |
| `init` | Writes `awesome.schema` and `migrations/` | `crates/e2e` offline_cli; CLI dispatch tests |
| `validate` | Provider required, at least one model or edge, `@id` rules, flexible objects, nested parents, edge `in`/`out` | `crates/core/src/validation.rs` |
| `format` | Formats the DSL; `--write` saves it | CLI command + use case tests |
| Parse | `datasource`, `naming`, `generator`, `model`, `edge`, `type`, field attributes in the example schema | `crates/parser` |
| Field assignments | `@default`, `@defaultAlways`, `@value`, `@updated`, `@readonly` render as `DEFAULT`, `DEFAULT ALWAYS`, `VALUE`, `READONLY` | `crates/renderers` |
| Ids and links | `id @id` → `record<table>`. `@link` → `REFERENCE` (+ `@onDelete`); opposite `Post[] @link("Name")` → `COMPUTED <~` | parser + renderer + introspect + e2e record_refs |
| Nested objects | `@flexible`, dotted paths, inline `{ ... }`, reusable `type` | parser + renderer tests |
| Naming | `naming { tables, fields }`. Omitted `fields` keeps declared names | parser + naming tests |
| Indexes (basic) | `@@index([fields])` and `@unique` render `DEFINE INDEX` | renderer tests |
| Index kinds | `@@index` `@unique` / `@fulltext("…")` / `@vector(N)` `@dist(…)` → UNIQUE, FULLTEXT ANALYZER BM25, HNSW | parser + renderer + introspect |
| Permissions (raw) | `@@permissions("FULL")` renders `DEFINE TABLE … PERMISSIONS` | renderer tests |
| `generate --target schema` | Prints full SurrealQL | CLI tests; e2e offline_cli |
| TypeScript codegen | Dual record/`*Selected` shapes; Create/Update inputs; nested `Select`/`GetPayload`; hybrid `*WhereInput` on `findMany`/`findUnique`; `orderBy` + `take`/`skip` on `findMany`; `createMany`/`updateMany`/`deleteMany`/`upsert` (`{ count }` or `select` via Surreal RETURN); `*WhereUniqueInput`; `$transaction` (interactive Surreal txn via `beginTransaction`); shared `buildProjection`/`buildWhere`/`buildOrderBy`; thin CRUD wrappers; `createClient`; writes `generator.output` unless `--stdout` | `crates/codegen-typescript`; CLI tests; live CRUD + select + where + order + bulk + transaction via `e2e/typescript` (Vitest). No zod; no live wrappers |
| `migrate dev` | Diffs models against the last snapshot and writes up, down, and `snapshot.json` | `crates/migrations` |
| `migrate create` | Empty migration directory | use case + CLI tests |
| `migrate status` | Applied vs pending via `_awesome_migrations` ledger; snapshot flag | use case tests; e2e migrate |
| `migrate apply` | Pending ups only; records name + checksum in ledger | use case tests; e2e migrate |
| `migrate rollback` | Runs `migration.down.surql` newest-first; removes ledger rows | use case tests; e2e migrate |
| `db push` | Renders SurrealQL and executes it with the official client | use case + CLI tests; e2e db_sync / graph_edges / record_refs |
| `db pull` | `INFO FOR DB` / `INFO FOR TABLE`, DSL write, `--force`, `--split-by-table` | renderer introspect + e2e db_sync |
| Table changes | Create, drop, and alter table mode; create, alter, and drop fields | differ + renderer |
| Index / permission removal | Differ emits `DropIndex` and `DropPermission`; renderer emits `REMOVE INDEX` / `PERMISSIONS NONE` | migrations + renderer tests |

## Partial

| Area | What works | Gap |
|---|---|---|
| `edge` blocks | Parsed and rendered as `DEFINE TABLE … TYPE RELATION IN … OUT …`; differ create/alter/drop; edge permissions; e2e roundtrip | `LIGHTWEIGHT` / `INLINE` not modeled |
| `@relation` | Parsed; must name an existing edge; skipped in SurrealQL (navigation-only for codegen) | Not restored by `db pull` (not stored in DB) |
| Events, functions | `CreateEvent`, `DropEvent`, `CreateFunction`, `DropFunction` render | No DSL syntax. Nothing in the parser produces them |
| Rust codegen | `generate --target rust` emits struct shells (optionals/arrays) | Still a stub: no dual link shapes, edges as real types, or client helpers |

## Not started

| Area | Evidence |
|---|---|
| Other providers | README lists MongoDB, Postgres, MySQL, SQLite. Only `renderers::surrealdb` exists |
| Assertions, live permissions model | Named in the README as SurrealDB features the DSL should be able to express. Not in the parser |
