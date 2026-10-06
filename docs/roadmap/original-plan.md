# Original plan

Source: scaffold commit `6fa7ad4` and the project README. There is no separate plan file from inception.

Awesome Schema is a Prisma-like DSL that stays close to the database. SurrealDB 3.x is the first target. The core stays database-agnostic so later providers do not rewrite the CLI or the migration engine.

## Milestone 1 — offline schema toolchain

The first milestone was meant to ship parsing, validation, diffing, and SurrealQL rendering. Live database commands were intentionally deferred.

- Cargo workspace with a hexagonal split: `cli` wires adapters, `core` owns the domain, and parser, migrations, renderers, and codegen sit behind ports.
- Commands: `init`, `validate`, `format`, `generate`, `migrate dev|create|status|apply`, `db pull`, `db push`.
- Domain model for tables, fields, indexes, edges, and migration operations.
- SurrealDB renderer for full schema DDL and migration plans.
- Filesystem migration store: `migration.surql`, snapshot, later also a down file.
- Codegen contracts for Rust and TypeScript, with stub generators.
- Planned providers after SurrealDB: MongoDB, Postgres, MySQL, SQLite.

`db pull`, `db push`, and `migrate apply` were specified as stubs until a connectivity milestone.

## What landed after that plan

These were not in the original offline milestone. They are already in `main`:

- Field assignments: `@default`, `@defaultAlways`, `@value`, `@updated`, `@readonly`.
- Down migrations from the reverse schema diff.
- Naming block; field names stay as written when `fields` is omitted.
- `id @id` infers `record<table>`. `RecordId<Model>` is rejected.
- `@flexible` objects, dotted nested fields, inline object blocks, reusable `type` definitions.
- `db push` and `migrate apply` execute SurrealQL through the official client.
- End-to-end tests against SurrealDB via testcontainers (`crates/e2e`). Those tests were not re-run in the latest check because they need Docker.
