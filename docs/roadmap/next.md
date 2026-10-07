# Next

Stay on SurrealDB until the schema that the DSL already accepts is what migrations and the database actually apply. Other providers wait until that loop is faithful.

## 1. Real graph edges — done (normal relations)

`edge` renders as `DEFINE TABLE … TYPE RELATION IN … OUT …`. `@relation` is navigation-only metadata that must name an existing edge (still skipped in SurrealQL). Differ includes edges. Still deferred:

- SurrealDB 3.3 `LIGHTWEIGHT` relations and `INLINE` edge fields. See [surrealdb-3.3.md](surrealdb-3.3.md).

## 2. Migration lifecycle — done

Applied migrations are recorded in SurrealDB table `_awesome_migrations` (same ns/db as the app).

- `migrate apply` runs only pending ups and stores a checksum.
- `migrate rollback [--steps N]` runs `migration.down.surql` newest-first, then removes ledger rows.
- `migrate status` reports applied vs pending (needs a live datasource).
- Differ emits `DropIndex` and `DropPermission` when indexes/permissions are removed.

## 3. `db pull` — done (lossy nav fields)

Introspects via `INFO FOR DB` / `INFO FOR TABLE`, writes DSL (`--force`, optional `--split-by-table`). `@relation` fields are not in the database, so pull does not recreate them.

## 4. Record references — done

`@link` always emits SurrealDB `REFERENCE` (default `ON DELETE IGNORE`). Prisma-style pairs share `@link("Name")`; the list side becomes `COMPUTED <~(table FIELD field)`. Use `@onDelete(Cascade|Unset|Reject|Ignore)` on the stored side. Dual-array many-to-many stays on `edge`.

## 5. Index kinds — done

`@@index([fields])` accepts trailing `@unique`, `@fulltext("analyzer")`, `@vector(N)`, and optional `@dist(Euclidean|Cosine|Manhattan)`.

- Full-text renders `FULLTEXT ANALYZER … BM25` (analyzer must already exist in the DB).
- Vector indexes render `HNSW DIMENSION … DIST …` (default Euclidean).
- Differ recreates indexes when params change.

## 6. Client generators — TypeScript client done (select + hybrid where + e2e)

TypeScript `generate --target typescript` writes to `generator.output` by default (`--stdout` print-only). Emits dual shapes, Create/Update inputs, nested `Select`/`GetPayload`, hybrid `*WhereInput` on `findMany`/`findUnique`, `orderBy` + `take`/`skip` on `findMany`, bulk/`upsert`/`return`, single `update`/`delete` opts, `count`/`groupBy`, one-hop nested writes (`create`/`connect`/`disconnect` on stored/computed `@link` and `@relation` edges via `RELATE`/`DELETE`), `*WhereUniqueInput`, `$transaction`, shared builders, thin CRUD/`query*` wrappers, and `createClient`. Live Vitest e2e covers CRUD, select, where, order, bulk, return, count/groupBy, nested writes, and transactions. Still deferred:

- Nested `set` / nested `update`/`delete`/`upsert` / `connectOrCreate`; multi-hop nested writes; nested bags on edge fluent API; `having`; top-level `aggregate()`; relation-field `count` select; `skipDuplicates`; single-script `BEGIN`/`COMMIT` helpers; nested tx / savepoints; cursor pagination; nested relation `orderBy`; multi-hop select paths; zod / runtime validators; live wrappers
- Rust generator rewrite (still a stub)

## 7. Later

- Events and functions in the DSL (operations already render).
- Assertions and a richer permissions model than a raw string.
- Providers: MongoDB, Postgres, MySQL, SQLite, each behind the existing renderer and introspector ports.

## Last live check

2026-10-07: `cargo test -p e2e --locked` against SurrealDB v3.3.0 via Docker; `cd e2e/typescript && npm test` — CRUD + select + where + order + bulk + return + count/groupBy + nested writes (links + edges) + `$transaction` against SurrealDB v3.3.0.
