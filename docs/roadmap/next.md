# Next

Stay on SurrealDB until the schema that the DSL already accepts is what migrations and the database actually apply. Other providers wait until that loop is faithful.

## 1. Real graph edges — done (normal relations)

`edge` renders as `DEFINE TABLE … TYPE RELATION IN … OUT …`. `@relation` is navigation-only metadata that must name an existing edge (still skipped in SurrealQL). Differ includes edges. Still deferred:

- SurrealDB 3.3 `LIGHTWEIGHT` relations and `INLINE` edge fields. See [surrealdb-3.3.md](surrealdb-3.3.md).

## 2. Migration lifecycle

`migrate apply` re-runs every `migration.surql`. Down files are unused.

- Record applied migrations (database ledger or local state the command can read).
- Apply only pending migrations.
- Add a rollback command that runs `migration.down.surql` in reverse order.
- Teach `migrate status` to report applied vs pending.
- Emit `DropIndex` (and permission removal) when the diff sees them.

## 3. `db pull` — done (lossy nav fields)

Introspects via `INFO FOR DB` / `INFO FOR TABLE`, writes DSL (`--force`, optional `--split-by-table`). `@relation` fields are not in the database, so pull does not recreate them.

## 4. Index kinds the domain already has

`Index.unique`, `fulltext`, and `vector` exist and the renderer can emit them. The parser never sets them.

- Extend `@@index` so those flags can be written in the DSL.
- Cover them in differ and renderer tests once the parser produces them.
- A vector index on 3.3 is `HNSW DIMENSION … DIST …`, not the bare `VECTOR` keyword the renderer appends today.

## 5. Client generators

Replace the Rust and TypeScript stubs after the schema and migrations match SurrealDB. Generated types should follow links, optionals, nested objects, and edges.

## 6. Later

- Events and functions in the DSL (operations already render).
- Assertions and a richer permissions model than a raw string.
- Providers: MongoDB, Postgres, MySQL, SQLite, each behind the existing renderer and introspector ports.

## Check still open

Re-run `cargo test -p e2e` with Docker available for live SurrealDB coverage.
