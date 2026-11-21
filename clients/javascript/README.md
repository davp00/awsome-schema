# awesome-schema

Alpha client runtime and CLI launcher for [Awesome Schema](https://github.com/davp00/awsome-schema) (`0.1.0-alpha.1`).

Generated TypeScript imports this package (`awesome-schema/runtime`). The npm tarball does not contain a native binary. On install, the launcher downloads the one CLI build that matches this machine from the GitHub Release for the same version (`v0.1.0-alpha.1`).

## Supported SurrealDB matrix

| Piece | Declaration |
|---|---|
| Servers | SurrealDB **3.3+** and **4.x** |
| Default e2e / Compose image | `surrealdb/surrealdb:v4` (`SURREAL_IMAGE` override for 3.3) |
| Rust SDK | crate `surrealdb = "4"` |
| JS SDK | peer `surrealdb@^2.2.2` |

See [docs/roadmap/surrealdb-matrix.md](../../docs/roadmap/surrealdb-matrix.md).

## Install → generate → e2e

```bash
pnpm add awesome-schema@alpha surrealdb@^2.2.2
# or local: AWESOME_SCHEMA_BIN=target/debug/awesome-schema pnpm awesome-schema validate --schema path/to.schema
pnpm awesome-schema generate --target typescript
pnpm --filter awesome-schema-typescript-e2e test   # default v4; set SURREAL_IMAGE for 3.3
```

`awesome-schema@test` is a manually published `1.0.0-test.<commit>` build (Actions → **publish-test-npm**). `awesome-schema@alpha` tracks tagged alphas. `AWESOME_SCHEMA_BIN` points the launcher at a binary you already have and skips the download.
