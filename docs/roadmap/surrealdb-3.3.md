# SurrealDB 3.3.0

Checked on 2026-10-06. Latest stable server and Rust SDK are **3.3.0** (24 Sep 2026). A later patch, 3.2.5 (5 Oct 2026), exists on the previous line. This repo tracks the 3.3 line.

## What was running before

| Piece | Before | Now |
|---|---|---|
| README target | 3.1.5 | 3.3.0 |
| Rust SDK (`surrealdb` crate) | 2.6.5 (manifest said `2.2`) | 3.3.0 |
| End-to-end container | `surrealdb/surrealdb:v2.2` (hardcoded in `testcontainers-modules` 0.15) | `v3.3.0` via `with_tag` |

SDK 2.x cannot open a WebSocket to a 3.x server (subprotocol mismatch). `Root` sign-in now takes owned `String` username and password.

## Schema features we already emit

These still match `DEFINE FIELD` on 3.3.0. No renderer change was required.

- `TYPE`, `option<T>`, `array<T>`, `record<table>`
- `FLEXIBLE` immediately after `TYPE`
- `DEFAULT`, `DEFAULT ALWAYS`, `VALUE`, `READONLY`
- `SCHEMAFULL` / `SCHEMALESS`
- `DEFINE INDEX … UNIQUE` and `FULLTEXT` (3.x name; 2.x called this `SEARCH ANALYZER`)
- Raw `PERMISSIONS`

`@value` and `@updated` stay `VALUE`. `COMPUTED` is a separate clause (evaluated on read, must be read-only). We do not emit it.

## Behaviour that changed under existing SQL

- `FLEXIBLE` now covers every object inside the type, including `option<object>`, `array<object>`, and object arms of a union. A type with no object is rejected. Our validator already requires `object` for `@flexible`.
- `DEFINE FIELD id … TYPE` is enforced when a record is written. Our `@id` fields render as `TYPE record<table>`, so an id that is not a record of that table fails on insert. `DEFAULT` on `id` is allowed. `DEFAULT ALWAYS` on `id` is rejected, because an explicit id must win. The DSL does not block `@defaultAlways` on `@id` yet.
- `DEFAULT`, `VALUE`, and `COMPUTED` run in dependency order, not field-name order. Interdependent fields no longer depend on declaration order.

## New in 3.2–3.3 that the DSL does not model

These belong on the graph-edge and index work in [next.md](next.md). They do not change SQL we generate today.

- `TYPE RELATION IN … OUT …`, plus `LIGHTWEIGHT` (record-less `in`/`out` only; no fields, indexes, events, or `SCHEMAFULL`).
- `INLINE` on a relation field, so a filtered traversal such as `->(likes WHERE score > 5)` can be answered from the adjacency entry.
- `INLINE EDGES` and `INLINE REFERENCES` on a vertex table.
- Vector indexes are `HNSW DIMENSION … DIST …` (or DiskANN), not a bare `VECTOR` keyword. The renderer still appends `VECTOR` when `Index.vector` is set. The parser never sets that flag, so current schemas do not emit it.
- Full-text `SEGMENT` tokenizer (Chinese, Japanese, Korean).
- `DEFINE ACCESS … CONTEXT` and `AUDIENCE`.
- `SELECT … FOR UPDATE`.

Engine-only changes (bitmap index combination, pre-filtered vector search, streaming results, Postgres wire protocol, gRPC) do not affect the DSL.
