# SurrealDB support matrix (3.3+ and 4.x)

Checked on 2026-11-20. Awesome Schema supports **two server lines** — SurrealDB **3.3+** and **4.x** — with **one client stack**. Do not pin marketing or docs to a single 4.x patch (e.g. “only 4.0.1”).

Historical 3.3-only notes: [surrealdb-3.3.md](surrealdb-3.3.md).

## How both versions work

| Piece | Declaration | Resolution |
|---|---|---|
| Supported servers | **3.3+** and **4.x** | — |
| Rust crate | `surrealdb = "4"` | `Cargo.lock` pins the exact build |
| JS peer | `surrealdb@^2.2.2` | lockfile resolves the patch |
| Default local image | `surrealdb/surrealdb:v4` via `SURREAL_IMAGE` | Override to `v3.3.0` for local 3.x |
| CI / e2e | Matrix: one job on 3.3 image, one on 4.x | Same tests, different `SURREAL_IMAGE` |

Official Rust SDK **4** connects to **3.x and 4.x** servers. JS `surrealdb` **2.2.x** documents support for servers from 2.0 through 4.x. We do **not** ship dual Cargo features (`surreal-3` / `surreal-4`).

## Pins and env

| Piece | Value | Where |
|---|---|---|
| Default Compose image | `${SURREAL_IMAGE:-surrealdb/surrealdb:v4}` | `docker-compose.yml` (storage `surrealkv:`) |
| TS e2e image | `process.env.SURREAL_IMAGE` or `surrealdb/surrealdb:v4` | `e2e/typescript/surreal/globalSetup.ts` |
| Rust e2e tag | from `SURREAL_IMAGE` / `SURREAL_TAG`, default `v4` | `crates/e2e/tests/surreal/common/mod.rs` |
| CI matrix | `surrealdb/surrealdb:v3.3.0` and `surrealdb/surrealdb:v4` | `.github/workflows/typescript-e2e.yml`, `rust-surreal-e2e.yml` |
| Rust SDK | `surrealdb = "4"` (`protocol-ws`) | workspace `Cargo.toml` |
| JS / npm SDK | `surrealdb@^2.2.2` | `clients/javascript`, `e2e/typescript` |

Soft-skip without Docker remains **local-only**. CI sets `CI=true` and fails both matrix cells when Surreal cannot start.

Local override examples:

```bash
SURREAL_IMAGE=surrealdb/surrealdb:v3.3.0 docker compose up -d
SURREAL_IMAGE=surrealdb/surrealdb:v3.3.0 cargo test -p e2e --locked --test surreal
SURREAL_IMAGE=surrealdb/surrealdb:v4 pnpm --filter awesome-schema-typescript-e2e test
```

## Intersection SurrealQL / runtime

Write once for both lines:

- Keep **ORDER BY** (and `ORDER BY id` for take/skip) — required on 4.x unordered scans; fine on 3.3.
- Do **not** emit 4-only DDL (`DEFINE TYPE`, migration clauses, COMPATIBILITY 4 assumptions) until explicitly modeled.
- Avoid 3.x-only quirks that 4 broke; fix when either CI cell fails.
- Query bind vars: do not use Surreal 4–protected names such as `parent` (runtime uses `as_parent` / `$as_parent`). Nested SELECT still uses language `$parent.field`.
- JS adapter: `encodeRecordIdString` escapes non-simple id parts as `table:⟨id⟩` so `StringRecordId` works on Surreal 4 (hyphenated / special ids).
- No separate codegen targets per Surreal major.

## Out of scope

- Mutually exclusive Cargo features for SDK 3 vs 4
- Surreal 2.x servers
- Postgres/MySQL wire as an Awesome Schema surface
- Surreal Cloud / Earth-only features
- Owning every 3→4 datastore upgrade path for end users (we connect to either server; we do not own their migration)
