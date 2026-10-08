# Contributing

Thanks for helping with Awesome Schema. The [README](README.md) is the startup guide. This file is how to build, test, and extend the repo.

## Prerequisites

- Rust stable, with the `rustfmt` and `clippy` components from [`rust-toolchain.toml`](rust-toolchain.toml)
- Docker, optional, for live SurrealDB via Compose or the end-to-end suites

## Workspace

This repository is a Cargo workspace:

```
crates/
├── cli                 # Binary: awesome-schema (orchestration only)
├── core                # Domain model, ports, use cases, errors
├── parser              # Awesome Schema DSL → domain model
├── migrations          # Schema diffing → migration plans
├── renderers           # Provider-specific DDL renderers (SurrealDB first)
├── codegen             # Language-agnostic codegen contracts
├── codegen-rust        # Rust model generator (stub)
└── codegen-typescript  # TypeScript client (shared runtime + thin typed wrappers)
```

`core` owns the database-agnostic domain: models, fields, indexes, edges, migration operations, port traits, and use cases. `parser`, `migrations`, `renderers`, and `codegen-*` implement or support those ports. `cli` wires dependencies in `di.rs`, implements adapters, and exposes commands through `clap`. Business logic stays out of the CLI.

Port traits in `core`:

- `SchemaRenderer` — render a full schema as provider DDL
- `MigrationRenderer` — render a `MigrationPlan` as executable statements
- `SchemaIntrospector` — pull live schema from a database
- `MigrationStore` — load and save schema snapshots for diffs
- `SchemaSource` — load the DSL file

What is done and what is next lives in [`docs/roadmap/status.md`](docs/roadmap/status.md) and [`docs/roadmap/next.md`](docs/roadmap/next.md).

## Coding agents

This repo keeps a [graphify](https://github.com/safishamsi/graphify) knowledge graph in [`graphify-out/`](graphify-out/). Use it before grepping the tree.

1. Read [`graphify-out/GRAPH_REPORT.md`](graphify-out/GRAPH_REPORT.md) for community hubs, god nodes, and the commit the graph was built from.
2. Answer architecture questions from the graph:

```bash
graphify query "how does migrate dev reach SurrealQL"
graphify explain "DatabaseSchema"
graphify path "GenerateCodeUseCase" "SurrealDbRenderer"
```

3. After code changes, refresh the graph (no API cost) and keep the result:

```bash
graphify update .
```

Commit `graphify-out/` with the change, or as a follow-up `docs(graphify)` commit. Do not discard those files.

Skills in [`.agents/skills/`](.agents/skills/) are the local conventions for an agent session:

- `awesome-feature` — plan first when a change crosses crates, live Surreal behavior, or a generated client
- `split-core-hexagonal-cli` — ports, use cases, and CLI adapters
- `rust-best-practices` and `rust-testing` — Rust style and tests
- `.cursor/rules/codegen-prefer-reusable.mdc` — shared helpers before copied emitter bodies

## Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
./scripts/check.sh
```

`./scripts/check.sh` is the full local gate: format, clippy, and `cargo test --workspace --locked` (including e2e).

Use [conventional commits](https://www.conventionalcommits.org/) (`feat`, `fix`, `docs`, `refactor`, and a scope when it helps).

## Tests

```bash
cargo test --workspace --locked --exclude e2e
cargo test -p e2e --locked --test surreal
./scripts/coverage.sh
```

Coverage uses `cargo llvm-cov` (`rustup component add llvm-tools-preview` and `cargo install cargo-llvm-cov`). Optional gate: `COVERAGE_FAIL_UNDER_LINES=90 ./scripts/coverage.sh`.

`cargo test -p e2e --test surreal` runs the Surreal CLI suite. Live SurrealDB soft-skips locally without Docker. A single case is `cargo test -p e2e --test surreal <name>`:

```bash
cargo test -p e2e --test surreal offline_cli   # init + generate (always runs)
cargo test -p e2e --test surreal migrate       # apply / status / rollback ledger
cargo test -p e2e --test surreal db_sync       # db push / pull / split-by-table
cargo test -p e2e --test surreal graph_edges   # TYPE RELATION roundtrip
cargo test -p e2e --test surreal record_refs   # @link REFERENCE + COMPUTED
```

TypeScript generated-client e2e uses Vitest. It starts SurrealDB **v3.3.0** via testcontainers unless `SURREALDB_URL` is set. It soft-skips locally when neither a URL nor Docker is available. GitHub Actions (`typescript-e2e`) sets `CI=true` and fails instead of skipping.

```bash
cd e2e/typescript && pnpm install && pnpm test

# Fast local loop against Compose:
docker compose up -d
cd e2e/typescript && pnpm test:external   # SURREALDB_URL=ws://127.0.0.1:8000

# Optional hard-fail locally (same as CI):
REQUIRE_SURREAL=1 pnpm test
```

Index-kind smoke test:

```bash
./scripts/manual-test-indexes.sh          # offline generate checks
./scripts/manual-test-indexes.sh --live   # compose up + push + INFO + pull
```

The alpha client package is `clients/javascript` (`awesome-schema`). `pnpm --filter awesome-schema test` runs its unit tests. From the repo root, `AWESOME_SCHEMA_BIN=target/debug/awesome-schema pnpm awesome-schema` runs a binary you just built. Other projects download one platform binary from the GitHub Release for the package version.

## Adding a database provider

`cli/src/di.rs` still constructs the Surreal renderer, executor, introspector, and migration ledger. `require_surreal_provider` refuses any other `datasource.provider` before schema SQL, migration SQL, and `db push`. TypeScript and Rust codegen return an error unless the provider is `surrealdb`. `validate` and `format` stay provider-agnostic.

A new provider is a new implementation, not a branch inside the Surreal emitter:

1. Implement `SchemaRenderer` and `MigrationRenderer` (in `renderers`, or a dedicated crate).
2. Map domain `MigrationOperation` variants to that provider's DDL.
3. Optionally implement `SchemaIntrospector`, a database executor, and a migration ledger for `db pull`, `db push`, and migrate apply.
4. Register those adapters from `datasource.provider`, and allow that provider through the same gate `require_surreal_provider` uses today.
5. Keep SurrealQL out of `core` and out of the shared TypeScript type printers.

Planned providers: `mongodb`, `postgres`, `mysql`, `sqlite`.
