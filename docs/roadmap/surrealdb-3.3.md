# SurrealDB 3.3.0 (historical)

Checked on 2026-11-20 during the 3.3-only stabilize cut. **Current policy** is dual support for **3.3+ and 4.x** — see [surrealdb-matrix.md](surrealdb-matrix.md).

This note keeps the 3.3 engine / DSL context that still applies on the 3.3 line and (where intersection allows) on 4.x.

## Former single-line matrix (superseded)

| Piece | Pin at 3.3 cut |
|---|---|
| Server image | `surrealdb/surrealdb:v3.3.0` |
| Rust SDK crate | `3.3.0` |
| JS / npm SDK | `surrealdb@2.0.10` (exact) |

## Schema features we already emit

These still match `DEFINE FIELD` on 3.3.0 and remain on the 3.3∩4.x intersection unless a CI cell proves otherwise.

- `TYPE`, `option<T>`, `array<T>`, `record<table>`
- `FLEXIBLE` immediately after `TYPE`
- `DEFAULT`, `DEFAULT ALWAYS`, `VALUE`, `READONLY`
- `SCHEMAFULL` / `SCHEMALESS`
- `DEFINE INDEX … UNIQUE` and `FULLTEXT ANALYZER … BM25`
- `DEFINE INDEX … HNSW DIMENSION … DIST …` for vector indexes
- Raw `PERMISSIONS`

`@value` and `@updated` stay `VALUE`. `COMPUTED` is a separate clause (evaluated on read, must be read-only). We do not emit it from those attributes.

## Behaviour notes (3.3)

- `FLEXIBLE` covers every object inside the type, including `option<object>`, `array<object>`, and object arms of a union.
- `DEFINE FIELD id … TYPE` is enforced when a record is written. `@id` fields render as `TYPE record<table>`.
- `INFO FOR TABLE` does **not** list the primary `id` field. `db pull` synthesizes `id @id`.
- `DEFAULT`, `VALUE`, and `COMPUTED` run in dependency order, not field-name order.

## New in 3.2–3.3 that the DSL does not model

Still deferred (see [next.md](next.md)):

- `LIGHTWEIGHT` relations; `INLINE` on relation fields; `INLINE EDGES` / `INLINE REFERENCES`
- DiskANN vector indexes; full-text `SEGMENT` tokenizer
- `DEFINE ACCESS … CONTEXT` / `AUDIENCE`; `SELECT … FOR UPDATE`
