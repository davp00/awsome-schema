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

## 6. Client generators — TypeScript client done (shallow select)

TypeScript `generate --target typescript` writes to `generator.output` by default (`--stdout` print-only). Emits dual shapes, Create/Update inputs, `GetPayload`/`Select`, CRUD + `query*` helpers, and `createClient` with shallow boolean `.select({ posts: true })` (FETCH). Still deferred:

- Nested select (`select: { posts: { select: { author: true } } }`)
- Zod / runtime validators; transaction / live / batch wrappers
- Rust generator rewrite (still a stub)

## 7. Later

- Events and functions in the DSL (operations already render).
- Assertions and a richer permissions model than a raw string.
- Providers: MongoDB, Postgres, MySQL, SQLite, each behind the existing renderer and introspector ports.

## Last live check

2026-10-07 on `c344710`: `cargo test -p e2e --locked` — 9 passed (`offline_cli`, `migrate`, `db_sync`, `graph_edges`, `record_refs`) against SurrealDB v3.3.0 via Docker. No soft-skips.
