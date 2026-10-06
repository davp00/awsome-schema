# Status

Checked on 2026-10-06 against `main` (`4141758`).

Check so far:

- Read the CLI, domain, parser, differ, SurrealDB renderer, codegen, and `db` adapters.
- `cargo test --workspace --exclude e2e` — **156 tests passed**, 0 failed.
- End-to-end tests in `crates/e2e` were **not** run. They start a SurrealDB container and need Docker.

Legend: **done** means the behavior exists and is covered by unit or CLI tests. **partial** means some of the path works and a specific gap remains. **not started** means the command or type exists but does not do the job.

## Done

| Area | What works | Where it was checked |
|---|---|---|
| Workspace layout | `cli`, `core`, `parser`, `migrations`, `renderers`, `codegen`, `codegen-rust`, `codegen-typescript`, `e2e` | `Cargo.toml` |
| `init` | Writes `awesome.schema` and `migrations/` | `crates/e2e` test exists; CLI dispatch tests |
| `validate` | Provider required, at least one model or edge, `@id` rules, flexible objects, nested parents, edge `in`/`out` | `crates/core/src/validation.rs` |
| `format` | Formats the DSL; `--write` saves it | CLI command + use case tests |
| Parse | `datasource`, `naming`, `generator`, `model`, `edge`, `type`, field attributes in the example schema | `crates/parser` — 58 tests |
| Field assignments | `@default`, `@defaultAlways`, `@value`, `@updated`, `@readonly` render as `DEFAULT`, `DEFAULT ALWAYS`, `VALUE`, `READONLY` | `crates/renderers` |
| Ids and links | `id @id` becomes `record<table>`. `User @link` becomes `record<user>`. Optionals use `option<T>` | parser + renderer tests |
| Nested objects | `@flexible`, dotted paths, inline `{ ... }`, reusable `type` | parser + renderer tests |
| Naming | `naming { tables, fields }`. Omitted `fields` keeps declared names | parser + naming tests |
| Indexes (basic) | `@@index([fields])` and `@unique` render `DEFINE INDEX` | renderer tests |
| Permissions (raw) | `@@permissions("FULL")` renders `DEFINE TABLE … PERMISSIONS` | renderer tests |
| `generate --target schema` | Prints full SurrealQL | CLI tests; e2e test exists |
| `migrate dev` | Diffs models against the last snapshot and writes up, down, and `snapshot.json` | `crates/migrations` — 7 tests |
| `migrate create` / `status` | Empty migration dir, and a filesystem listing plus snapshot flag | use case tests |
| Table changes | Create, drop, and alter table mode; create, alter, and drop fields | differ + renderer |

## Partial

| Area | What works | Gap |
|---|---|---|
| `edge` blocks | Parsed (`in`, `out`, fields, table mode) and rendered as `DEFINE TABLE` plus fields | Not `TYPE RELATION IN … OUT …`. `in` / `out` are stored and never emitted. `migrate dev` diffs models only, so edge changes produce no operations |
| `@relation` | Parsed, for example `posts Post[] @relation("user_posts")` | Renderer skips any field with `relation_name`. The example `posts` field does not appear in SurrealQL |
| Indexes beyond a field list | Domain `Index` has `unique`, `fulltext`, `vector`. Renderer can emit them if those flags are set | Parser always sets `unique`, `fulltext`, and `vector` to `false` for `@@index` |
| Index removal | `DropIndex` exists as a migration operation and the renderer emits `REMOVE INDEX` | The differ never emits `DropIndex` when an index disappears |
| Events, functions | `CreateEvent`, `DropEvent`, `CreateFunction`, `DropFunction` render | No DSL syntax. Nothing in the parser produces them |
| `db push` | Loads the schema, renders SurrealQL, executes it with the official client | README still calls this a stub. Not re-checked against a live database in this pass |
| `migrate apply` | Reads each `migration.surql` and executes non-empty scripts | No applied-migration ledger, so a second apply runs the same files again. Down files are never executed. Not re-checked live in this pass |
| `migrate status` | Lists migration directory names and whether a snapshot exists | Does not know what the database has applied |
| Codegen | `generate --target rust` and `typescript` emit struct / type shells, including optionals and arrays | Source marks them as stubs. No links, edges, nested objects as real types, or a client |

## Not started

| Area | Evidence |
|---|---|
| `db pull` | Introspects via `INFO FOR DB` / `INFO FOR TABLE`, merges with on-disk config blocks, writes DSL (`--force`, optional `--split-by-table`) | partial — `@relation` / edge endpoints still lossy |
| Rollback command | Down files are written only |
| Other providers | README lists MongoDB, Postgres, MySQL, SQLite. Only `renderers::surrealdb` exists |
| Assertions, vector index syntax, live permissions model | Named in the README as SurrealDB features the DSL should be able to express. Not in the parser |

## README drift

The README still says `db push` and `migrate apply` are stubs. The code and `crates/e2e/tests/surrealdb_workflow.rs` treat both as real commands. `db pull` is still a stub, and that part of the README is accurate.
