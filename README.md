# Awesome Schema

Awesome Schema is a **schema modeling, migration, and code generation toolkit** for modern databases. It is inspired by [Prisma](https://www.prisma.io/) and [TypeORM](https://typeorm.io/), but designed from the ground up to express database-native capabilities—starting with **SurrealDB 3.x**.

Define your schema once in an `awesome.schema` file, validate it, generate SurrealQL, track migrations, and (eventually) generate client models for Rust, TypeScript, and more.

## Why Awesome Schema exists

ORMs and schema tools often flatten databases into the lowest common denominator. SurrealDB has rich features—graph edges, record links, permissions, events, assertions, vector indexes—that are awkward or impossible to model faithfully in generic tools.

Awesome Schema provides a **clean modeling layer** that feels familiar (Prisma-like DSL) while staying close to SurrealDB's native schema concepts. The architecture is **database-agnostic at the core**, so additional providers can be added without rewriting the CLI or migration engine.

## How it differs from Prisma and TypeORM

| | Prisma | TypeORM | Awesome Schema |
|---|---|---|---|
| Primary model | Relational tables + Prisma Client | Decorator-based entities + migrations | DSL → domain model → provider renderers |
| SurrealDB support | Limited / indirect | Manual | First-class target (SurrealDB 3.3.0) |
| Graph edges | Relations | Relations | Native `edge` blocks |
| Migration model | SQL migrations (provider-specific) | TypeORM migration classes | Domain operations → SurrealQL |
| Code generation | Strong client focus | Entity classes | Pluggable per-language generators |

## Why SurrealDB first

SurrealDB 3.x is the initial target because it combines document, graph, and schema-full/schemaless tables in one engine. Awesome Schema embraces that expressiveness rather than hiding it. The current target is **SurrealDB 3.3.0**. The first milestone started on 3.1.5.

Runtime database connectivity (`db pull`, `db push`, `migrate apply`) is intentionally stubbed—this milestone focuses on **parsing, validation, diffing, and SurrealQL rendering**.

## Workspace structure

This repository is a **Cargo workspace** with clean crate boundaries:

```
crates/
├── cli                 # Binary: awesome-schema (orchestration only)
├── core                # Domain model, ports, use cases, errors
├── parser              # Awesome Schema DSL → domain model
├── migrations          # Schema diffing → migration plans
├── renderers           # Provider-specific DDL renderers (SurrealDB first)
├── codegen             # Language-agnostic codegen contracts
├── codegen-rust        # Rust model generator (stub)
└── codegen-typescript  # TypeScript model generator (stub)
```

### Architecture (hexagonal / clean)

- **`core`** owns the database-agnostic domain: models, fields, indexes, edges, migration operations, port traits, and use cases.
- **`parser`**, **`migrations`**, **`renderers`**, and **`codegen-*`** are capability crates that implement or support ports.
- **`cli`** wires dependencies via `di.rs`, implements adapters (filesystem, migration store, introspector stubs), and exposes commands through `clap`. **No business logic lives in the CLI.**

Port traits (in `core`):

- `SchemaRenderer` — render a full schema as provider DDL
- `MigrationRenderer` — render a `MigrationPlan` as executable statements
- `SchemaIntrospector` — pull live schema from a database
- `MigrationStore` — load/save schema snapshots for diffs
- `SchemaSource` — load the DSL file

## Getting started

### Prerequisites

- Rust **2024 edition** (stable toolchain via `rust-toolchain.toml`)

### Build and test

```bash
cargo check --workspace
cargo test --workspace
```

### CLI usage

```bash
# Initialize a new project (creates awesome.schema + migrations/)
cargo run -p cli -- init

# Validate schema
cargo run -p cli -- validate
cargo run -p cli -- --schema examples/awesome.schema validate

# Format schema (use --write to save)
cargo run -p cli -- format
cargo run -p cli -- format --write

# Generate SurrealQL or client stubs
cargo run -p cli -- generate
cargo run -p cli -- generate --target rust
cargo run -p cli -- generate --target typescript

# Migrations
cargo run -p cli -- migrate dev
cargo run -p cli -- migrate create <name>
cargo run -p cli -- migrate status
cargo run -p cli -- migrate apply   # stub — requires DB connectivity

# Database commands (stubs)
cargo run -p cli -- db pull
cargo run -p cli -- db push
```

Global flags:

- `--schema <path>` — schema file (default: `awesome.schema`)
- `--migrations-dir <path>` — migrations directory (default: `migrations`)

## Awesome Schema DSL

See [`examples/awesome.schema`](examples/awesome.schema) for a full example. The DSL supports (or is designed to support):

- Namespaces and databases (via `datasource`)
- `model` and `edge` blocks
- Schemafull / schemaless tables (`@@table`)
- Fields, optionals, arrays, record links, relations
- Flexible nested objects (`object @flexible`) with inline blocks or reusable `type` definitions
- Field assignments: `@default`, `@value`, `@updated`, `@readonly`
- Indexes, permissions, and provider-specific extensions

Field assignments map to SurrealDB `DEFINE FIELD` clauses:

| DSL attribute | SurrealDB clause | Behavior |
|---|---|---|
| `@default(expr)` | `DEFAULT expr` | Applied on INSERT when no value is provided |
| `@defaultAlways(expr)` | `DEFAULT ALWAYS expr` | Also applied on UPDATE when the value is empty |
| `@value(expr)` | `VALUE expr` | Recomputed on every CREATE and UPDATE |
| `@updated(expr)` | `VALUE expr` | Alias for auto-updating timestamps (`updatedAt`) |
| `@readonly` | `READONLY` | Prevents manual updates (use with `@value`) |
| `@flexible` | `FLEXIBLE` | Allows extra undefined keys on a schemafull object field |

Flexible objects mirror SurrealDB's schemaless-in-schemafull pattern for nested data. Define nested fields inline, with dotted paths, or via a reusable type:

```prisma
// Inline object block
model User {
  id        @id
  metadata  object @flexible {
    user_id int?
    source  string
  }
}

// Reusable object type
type UserMetadata @flexible {
  user_id int?
  source  string
}

model User {
  id        @id
  metadata  UserMetadata
}
```

Both forms render as:

```sql
DEFINE FIELD metadata ON user TYPE object FLEXIBLE;
DEFINE FIELD metadata.user_id ON user TYPE option<int>;
DEFINE FIELD metadata.source ON user TYPE string;
```

Dotted paths (`metadata.user_id int?`) remain supported for flat declarations.

```prisma
model User {
  id        @id
  email     string @unique
  createdAt datetime @value(time::now()) @readonly
  updatedAt datetime @updated(time::now())
  posts     Post[] @relation("user_posts")

  @@table(schemafull)
}
```

Model `@id` fields infer a SurrealDB `record<table>` type from the containing model — no `RecordId<Model>` syntax needed.

```prisma
edge Likes {
  in  User
  out Post
  @@table(schemafull)
}
```

## Migrations

Migrations work similarly to **TypeORM**:

1. `migrate dev` parses the current schema and diffs it against the latest `snapshot.json`.
2. Domain-level operations are produced (`CreateTable`, `CreateField`, `CreateIndex`, …).
3. The SurrealDB renderer writes `migration.surql` (up) and `migration.down.surql` (down).
4. A new `snapshot.json` is saved for the next diff.

Down migrations are computed as the reverse schema diff (`current → previous`), so rollback operations mirror the forward migration.

Example layout:

```
migrations/
└── 20260701183000_create_user_and_post/
    ├── migration.surql
    ├── migration.down.surql
    └── snapshot.json
```

Each migration directory contains:

- `migration.surql` — forward (up) operations to apply the schema change
- `migration.down.surql` — reverse (down) operations for rollback
- `snapshot.json` — normalized schema state after the up migration

See [`examples/migrations/`](examples/migrations/) for a reference migration.

## Adding a new database provider

1. Implement `SchemaRenderer` and `MigrationRenderer` in `renderers` (e.g. `renderers::postgres`) or a dedicated `crates/providers/<name>` crate.
2. Map domain `MigrationOperation` variants to provider DDL.
3. Optionally implement `SchemaIntrospector` for `db pull`.
4. Register the adapter in `cli/src/di.rs` based on `datasource.provider`.
5. Keep SurrealDB-specific logic out of `core`, `migrations`, and `cli` commands.

Planned providers: `surrealdb`, `mongodb`, `postgres`, `mysql`, `sqlite`.

## Development

```bash
# Format and lint
cargo fmt --all
cargo clippy --workspace --all-targets

# Run helper script (if configured)
./scripts/check.sh
```

## License

MIT
