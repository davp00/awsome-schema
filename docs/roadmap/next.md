# Next

Stay on SurrealDB until the schema that the DSL already accepts is what migrations and the database actually apply. Other providers wait until that loop is faithful.

## 1. Real graph edges — done (normal relations)

`edge` renders as `DEFINE TABLE … TYPE RELATION IN … OUT …`. `@relation` is navigation-only metadata that must name an existing edge (still skipped in SurrealQL). Differ includes edges. Still deferred:

- SurrealDB `LIGHTWEIGHT` relations and `INLINE` edge fields. See [surrealdb-matrix.md](surrealdb-matrix.md) / [surrealdb-3.3.md](surrealdb-3.3.md).

## 2. Migration lifecycle — done

Applied migrations are recorded in SurrealDB table `_awesome_migrations` (same ns/db as the app).

- `migrate apply` runs only pending ups and stores a checksum.
- `migrate rollback [--steps N]` runs `migration.down.surql` newest-first, then removes ledger rows.
- `migrate status` reports applied vs pending (needs a live datasource).
- Differ emits `DropIndex` and `DropPermission` when indexes/permissions are removed.

## 3. `db pull` — done

Introspects via `INFO FOR DB` / `INFO FOR TABLE`, writes DSL (`--force`, optional `--split-by-table`). `@relation` nav fields are synthesized from RELATION edges (preserve names when present). `db push` fails if a `@fulltext("…")` analyzer is missing from `INFO FOR DB`.

## 4. Record references — done

`@link` always emits SurrealDB `REFERENCE` (default `ON DELETE IGNORE`). Prisma-style pairs share `@link("Name")`; the list side becomes `COMPUTED <~(table FIELD field)`. Use `@onDelete(Cascade|Unset|Reject|Ignore)` on the stored side. Dual-array many-to-many stays on `edge`.

## 5. Index kinds — done

`@@index([fields])` accepts trailing `@unique`, `@fulltext("analyzer")`, `@vector(N)`, and optional `@dist(Euclidean|Cosine|Manhattan)`.

- Full-text renders `FULLTEXT ANALYZER … BM25` (analyzer must already exist in the DB).
- Vector indexes render `HNSW DIMENSION … DIST …` (default Euclidean).
- Differ recreates indexes when params change.

## 6. Client generators — TypeScript client done (select + hybrid where + e2e)

TypeScript `generate --target typescript` dispatches on `datasource.provider` and only `surrealdb` emits a client. Schema TypeScript types are shared; only the Surreal module emits SQL and `SurrealOpsLike`. TypeScript and CLI e2e are keyed by provider, and only `surreal` exists. Schema SQL, migration SQL, `db push`, and Rust codegen run only for `surrealdb`. It writes to `generator.output` by default (`--stdout` print-only). Emits dual shapes, Create/Update inputs, nested `Select`/`GetPayload` with relation-bag `orderBy` + `take`/`skip`, hybrid `*WhereInput` on `findMany`/`findUnique`, top-level `orderBy` + `take`/`skip` on `findMany` (scalars + list link/edge `{ _count: "asc"|"desc" }`), bulk/`upsert`/`return`, single `update`/`delete` opts, `count`/`groupBy` (optional `having` filters the grouped rows), one-hop nested writes (`create`/`connect`/`disconnect`/`set`/`update`/`delete`/`upsert`/`connectOrCreate` on stored/computed `@link` and `@relation` edges), `*WhereUniqueInput`, `$transaction`, `$queryRaw` / `$executeRaw` (session and transaction client), session `live()` / `$live` (`LIVE SELECT`), shared builders, thin CRUD/`query*` wrappers, and `createClient`. Unmatched inline objects with children emit named record types (e.g. `UserPrefs`) shared with where `equals` / nested filters; bare `object` without children stays `Record<string, unknown>`. The shared Surreal runtime lives in the alpha npm package `awesome-schema` (`clients/javascript`, `0.1.0-alpha.1`). Generated code imports it. The tarball has no native binary; install downloads the one GitHub Release CLI that matches the machine, and `pnpm awesome-schema` runs it. Live Vitest e2e covers CRUD, select, where, order (incl. relation `_count`), nested relation orderBy, nested relation take/skip, bulk, return, count/groupBy, nested writes, transactions, raw query, and live select. Still deferred:

- Multi-hop nested writes; nested bags on edge fluent API; top-level `aggregate()`; relation-field `count` select; `skipDuplicates`; single-script `BEGIN`/`COMMIT` helpers; nested tx / savepoints; cursor pagination; multi-hop select paths; zod / runtime validators
- Rust generator rewrite (still a stub)

## 7. Later

- Events and functions in the DSL (operations already render).
- Assertions and a richer permissions model than a raw string.
- Providers: MongoDB, Postgres, MySQL, SQLite, each behind the existing renderer and introspector ports.

## 8. CLI distribution — done

- crates.io package rename (`awesome-schema` + `awesome-schema-*` libs) so `cargo install awesome-schema` works after tagged publish.
- `scripts/install.sh` installs the matching GitHub Release binary to `~/.local/bin`.
- `awesome-schema upgrade` replaces the running binary from GitHub Releases (or `--from crates-io`).

## Last live check

2026-11-20: Dual Surreal matrix (`0.1.0-alpha.1`). Rust `surrealdb = "4"`, JS `^2.2.2`. `cargo test -p e2e --test surreal` green on `SURREAL_IMAGE` **v3.3.0** and **v4**; `pnpm --filter awesome-schema-typescript-e2e test` 51/51 on both; unit `pnpm --filter awesome-schema test` green. Docs: [surrealdb-matrix.md](surrealdb-matrix.md).
