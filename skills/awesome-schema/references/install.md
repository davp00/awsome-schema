# Install and upgrade

Repo: `https://github.com/davp00/awsome-schema` (spelling **awsome**).

## Install the CLI

```bash
# crates.io (builds from source; needs Rust)
cargo install awesome-schema

# Prebuilt binary (macOS / Linux) → ~/.local/bin by default
curl -fsSL https://raw.githubusercontent.com/davp00/awsome-schema/main/scripts/install.sh | sh
# Optional: VERSION=0.1.0-alpha.1 INSTALL_DIR=$HOME/.local/bin

# npm / pnpm (downloads matching GitHub Release binary on install)
pnpm add -D awesome-schema@alpha surrealdb@2.0.10
# then: pnpm awesome-schema <command>
```

Override the binary without downloading:

```bash
AWESOME_SCHEMA_BIN=/path/to/awesome-schema pnpm awesome-schema validate
```

## Global flags

```bash
awesome-schema --schema path/to.schema --migrations-dir migrations <command>
```

Defaults: `--schema awesome.schema`, `--migrations-dir migrations`.

## Common commands

```bash
awesome-schema init
awesome-schema validate
awesome-schema format --write
awesome-schema generate --target schema          # SurrealQL to stdout
awesome-schema generate --target typescript
awesome-schema generate --target typescript --stdout
awesome-schema db push
awesome-schema db pull --force
awesome-schema db pull --split-by-table --force
awesome-schema migrate dev [--name <slug>]
awesome-schema migrate create <name>
awesome-schema migrate status
awesome-schema migrate apply
awesome-schema migrate rollback [--steps N]
awesome-schema upgrade [--check] [--version <ver>] [--from github|crates-io]
```

## Upgrade rules

| How you installed | How to update |
|---|---|
| curl / direct binary | `awesome-schema upgrade` |
| `cargo install` | `awesome-schema upgrade` or `upgrade --from crates-io` |
| npm / pnpm package | `pnpm update awesome-schema` / `npm update awesome-schema` only |

`upgrade` refuses npm-cache and `node_modules` paths on purpose.

## SurrealDB local

```bash
docker compose up -d          # surrealdb/surrealdb:v3.3.0
# url ws://127.0.0.1:8000, user/pass root/root
```

## Matrix

| Piece | Pin |
|---|---|
| Server | `v3.3.0` |
| JS SDK | `surrealdb@2.0.10` |
| Package cut | `0.1.0-alpha.1` |
