# Next

Stay on SurrealDB until the schema that the DSL already accepts is what migrations and the database actually apply. Other providers wait until that loop is faithful.

## 1. Real graph edges

`edge` and `@relation` parse today and then disappear or become ordinary tables.

- Render `edge` as `DEFINE TABLE … TYPE RELATION IN <in> OUT <out>`.
- Stop skipping `@relation` fields, or turn them into the edge they name.
- Include edges in `SchemaDiffer` (create, alter, drop), including edge fields.
- SurrealDB 3.3 adds `LIGHTWEIGHT` relations and `INLINE` edge fields. See [surrealdb-3.3.md](surrealdb-3.3.md). Model those only after a normal relation renders correctly.

## 2. Migration lifecycle

`migrate apply` re-runs every `migration.surql`. Down files are unused.

- Record applied migrations (database ledger or local state the command can read).
- Apply only pending migrations.
- Add a rollback command that runs `migration.down.surql` in reverse order.
- Teach `migrate status` to report applied vs pending.
- Emit `DropIndex` (and permission removal) when the diff sees them.

## 3. `db pull`

The introspector is the remaining explicit stub.

- Read `INFO FOR DB` / `INFO FOR TABLE` from SurrealDB.
- Map tables, fields, indexes, and relation tables back into `DatabaseSchema`.
- Decide how a pulled schema is written (`awesome.schema`, a snapshot, or both).

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
- Fix the README line that still calls `db push` and `migrate apply` stubs.

## Check still open

Re-run `cargo test -p e2e` with Docker available. That is the live check for `migrate apply`, `db push`, `init`, and the `db pull` failure. The 2026-10-06 pass only ran `cargo test --workspace --exclude e2e`.
