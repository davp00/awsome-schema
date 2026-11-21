---
name: awesome-schema
description: >-
  Install and use Awesome Schema for SurrealDB: CLI install (cargo, curl, npm),
  awesome.schema DSL (models, edges, @link, @relation), validate/format, db push/pull,
  migrate, and TypeScript client generation via awesome-schema/runtime. Use when the
  user mentions awesome-schema, Awesome Schema, SurrealDB schema DSL, cargo install
  awesome-schema, pnpm awesome-schema, db push, db pull, migrate apply, generate
  typescript client, or schema modeling for SurrealDB.
license: MIT
metadata:
  version: 0.1.0-alpha.1
  repository: https://github.com/davp00/awsome-schema
---

# Awesome Schema

Consumer skill for **using** Awesome Schema against SurrealDB. Not for maintaining the Awesome Schema repo itself.

## Supported matrix (do not invent outside this)

| Piece | Declaration |
|---|---|
| SurrealDB servers | **3.3+** and **4.x** |
| Default local image | `surrealdb/surrealdb:v4` (`SURREAL_IMAGE` for 3.3) |
| JS peer SDK | `surrealdb@^2.2.2` |
| npm / crates cut | `0.1.0-alpha.1` |

One client stack talks to both lines. Do not invent 4-only DDL or pin a single 4.x patch in consumer guidance.

## When to use

- New or existing `awesome.schema` / Surreal schema workflow
- Install or upgrade the CLI
- `db push` / `db pull` / migrate
- Generate a TypeScript Surreal client

## Hard rules

1. Prefer documented install paths in [references/install.md](references/install.md). Today that means `cargo install --git …` or npm; do not assume crates.io until a version is published there.
2. npm-managed CLIs: update with `pnpm update awesome-schema` / `npm update` — **never** `awesome-schema upgrade` on the npm cache binary.
3. Pass `--schema <path>` when the file is not `./awesome.schema`.
4. TypeScript generated code imports `awesome-schema/runtime`; peer `surrealdb@^2.2.2`.
5. `@relation` is navigation-only (must name an existing `edge`); `@link` is the stored/computed record reference.

## Workflow

### 1. Detect the goal

| Goal | Start at |
|---|---|
| Greenfield project | Install → `init` → edit DSL |
| Schema only | Edit → `validate` / `format` |
| Live DB sync | `db push` or migrate apply |
| Introspect DB | `db pull --force` |
| App client | `generate --target typescript` |

### 2. Ensure CLI + Surreal

See [references/install.md](references/install.md). Local Surreal:

```bash
docker compose up -d   # 4.x line at ws://127.0.0.1:8000 (root/root)
# SURREAL_IMAGE=surrealdb/surrealdb:v3.3.0 docker compose up -d
```

Datasource needs `provider = "surrealdb"`, `url`, `namespace`, `database`.

### 3. Schema loop

```bash
awesome-schema init
awesome-schema validate
awesome-schema format --write
# then either:
awesome-schema db push
# or:
awesome-schema migrate dev --name <change>
awesome-schema migrate apply
```

DSL essentials: [references/dsl.md](references/dsl.md). Minimal example: [examples/minimal.schema](examples/minimal.schema).

### 4. TypeScript client

```bash
awesome-schema generate --target typescript
```

Writes to `generator.output`. Details: [references/typescript-client.md](references/typescript-client.md).

### 5. Upgrade

- Standalone / curl: `awesome-schema upgrade` or `upgrade --check`
- crates.io reinstall: `awesome-schema upgrade --from crates-io`
- npm: package manager update only

## Done checks

- [ ] `awesome-schema validate` succeeds
- [ ] Live DB path: push or migrate applied; `migrate status` sane when using migrations
- [ ] TS client generates and imports `awesome-schema/runtime`
- [ ] Surreal / JS pins match the matrix above

## More detail

- [references/install.md](references/install.md)
- [references/dsl.md](references/dsl.md)
- [references/typescript-client.md](references/typescript-client.md)
- Upstream README: https://github.com/davp00/awsome-schema
