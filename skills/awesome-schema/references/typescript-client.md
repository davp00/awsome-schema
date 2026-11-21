# TypeScript generated client

## Prerequisites

```bash
pnpm add awesome-schema@alpha surrealdb@^2.2.2
```

Supported servers: SurrealDB **3.3+** and **4.x**. Peer **`surrealdb@^2.2.2`**.

Schema must include:

```prisma
generator client {
  provider = "typescript"
  output   = "./generated"   # or your path
}
```

## Generate

```bash
awesome-schema generate --target typescript
# print only:
awesome-schema generate --target typescript --stdout
```

Writes into `generator.output`. Generated modules import **`awesome-schema/runtime`**.

## Runtime expectations

- Use `createClient` / generated helpers with a Surreal session that matches the matrix.
- One-hop nested writes (`create` / `connect` / `disconnect` / `set` / `update` / `delete` / `upsert` / `connectOrCreate`) on stored/computed `@link` and `@relation` edges.
- Hybrid where, order/take/skip, `$transaction`, `$queryRaw` / `$executeRaw`, `live()` / `$live`.
- Record id / link filters use Surreal `type::record` semantics in the runtime — do not invent string-equality for record ids.

## Stale generated clients

If the app imports a checked-in generated tree, regenerate after schema changes. Prefer generating into `generator.output` and treating that folder as build output unless the project documents otherwise.

## CLI via npm

```bash
pnpm awesome-schema validate
pnpm awesome-schema generate --target typescript
```

The npm package downloads one platform binary from the GitHub Release for the package version. Override with `AWESOME_SCHEMA_BIN`.
