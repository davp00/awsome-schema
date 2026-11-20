# SurrealDB 3.3.0

Checked on 2026-11-20. This repo’s **supported matrix** is SurrealDB **v3.3.0** end-to-end. A later patch on the 3.2 line (3.2.5) exists; we stay on 3.3.

## Supported 3.3 matrix

| Piece | Pin | Where |
|---|---|---|
| Server image | `surrealdb/surrealdb:v3.3.0` | `e2e/typescript/surreal/globalSetup.ts` (`SURREAL_IMAGE`); Rust e2e `with_tag("v3.3.0")` in `crates/e2e/tests/surreal/common/mod.rs` |
| Rust SDK crate | `3.3.0` | workspace `Cargo.toml` (`surrealdb` with `protocol-ws`) |
| JS / npm SDK | `surrealdb@2.0.10` (exact) | `clients/javascript` peer + devDependency; `e2e/typescript` devDependency |

The JS SDK stays on the **2.x npm line** (`2.0.10`), which speaks WebSocket to a 3.3 server. That is intentional: do not bump the npm package to 3.x unless e2e is re-proven. Soft-skip without Docker is **local-only**; CI (`typescript-e2e` with `CI=true`) and PR checklist (`cargo test -p e2e --locked --test surreal`) must run live against this matrix.

## What was running before

| Piece | Before | Now |
|---|---|---|
| README target | 3.1.5 | 3.3.0 |
| Rust SDK (`surrealdb` crate) | 2.6.5 (manifest said `2.2`) | 3.3.0 |
| End-to-end container | `surrealdb/surrealdb:v2.2` (hardcoded in `testcontainers-modules` 0.15) | `v3.3.0` via `with_tag` |
| JS npm SDK | `^2.0.0` / `^2.0.10` | exact `2.0.10` |

Official Rust SDK 2.x cannot open a WebSocket to a 3.x server (subprotocol mismatch). `Root` sign-in now takes owned `String` username and password.

## Schema features we already emit

These still match `DEFINE FIELD` on 3.3.0. No renderer change was required.

- `TYPE`, `option<T>`, `array<T>`, `record<table>`
- `FLEXIBLE` immediately after `TYPE`
- `DEFAULT`, `DEFAULT ALWAYS`, `VALUE`, `READONLY`
- `SCHEMAFULL` / `SCHEMALESS`
- `DEFINE INDEX … UNIQUE` and `FULLTEXT ANALYZER … BM25` (3.x name; 2.x called this `SEARCH ANALYZER`)
- `DEFINE INDEX … HNSW DIMENSION … DIST …` for vector indexes
- Raw `PERMISSIONS`

`@value` and `@updated` stay `VALUE`. `COMPUTED` is a separate clause (evaluated on read, must be read-only). We do not emit it.

## Behaviour that changed under existing SQL

- `FLEXIBLE` now covers every object inside the type, including `option<object>`, `array<object>`, and object arms of a union. A type with no object is rejected. Our validator already requires `object` for `@flexible`.
- `DEFINE FIELD id … TYPE` is enforced when a record is written. Our `@id` fields render as `TYPE record<table>`, so an id that is not a record of that table fails on insert. `DEFAULT` on `id` is allowed. `DEFAULT ALWAYS` on `id` is rejected, because an explicit id must win. The DSL does not block `@defaultAlways` on `@id` yet.
- `INFO FOR TABLE` does **not** list the primary `id` field. `db pull` synthesizes `id @id` for models so validation still passes.
- `DEFAULT`, `VALUE`, and `COMPUTED` run in dependency order, not field-name order. Interdependent fields no longer depend on declaration order.

## New in 3.2–3.3 that the DSL does not model

These belong on the graph-edge and index work in [next.md](next.md). They do not change SQL we generate today.

- `TYPE RELATION IN … OUT …`, plus `LIGHTWEIGHT` (record-less `in`/`out` only; no fields, indexes, events, or `SCHEMAFULL`).
- `INLINE` on a relation field, so a filtered traversal such as `->(likes WHERE score > 5)` can be answered from the adjacency entry.
- `INLINE EDGES` and `INLINE REFERENCES` on a vertex table.
- Vector indexes are `HNSW DIMENSION … DIST …` (or DiskANN). The renderer emits HNSW when `Index.vector` is set (via `@@index([f]) @vector(N)`). DiskANN is not modeled.
- Full-text `SEGMENT` tokenizer (Chinese, Japanese, Korean).
- `DEFINE ACCESS … CONTEXT` and `AUDIENCE`.
- `SELECT … FOR UPDATE`.

Engine-only changes (bitmap index combination, pre-filtered vector search, streaming results, Postgres wire protocol, gRPC) do not affect the DSL.
