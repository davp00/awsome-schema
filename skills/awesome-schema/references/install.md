# Install and upgrade

Repo: `https://github.com/davp00/awsome-schema` (spelling **awsome**).

## Install the CLI

```bash
# From git (works without crates.io; needs Rust)
cargo install --git https://github.com/davp00/awsome-schema --locked awesome-schema

# npm / pnpm (downloads matching GitHub Release binary on install)
pnpm add -D awesome-schema@alpha surrealdb@^2.2.2
# then: pnpm awesome-schema <command>

# Prebuilt binary (macOS / Linux) → ~/.local/bin — needs a Release with assets
VERSION=0.1.0-alpha.1 curl -fsSL https://raw.githubusercontent.com/davp00/awsome-schema/main/scripts/install.sh | sh
# Or omit VERSION once a non-canary GitHub Release exists

# crates.io (only after the first tagged crates.io publish)
# cargo install awesome-schema
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
| `cargo install --git …` / curl binary | `awesome-schema upgrade` |
| `cargo install` from crates.io (when published) | `upgrade --from crates-io` |
| npm / pnpm package | `pnpm update awesome-schema` / `npm update awesome-schema` only |

`upgrade` refuses npm-cache and `node_modules` paths on purpose.

## SurrealDB local

```bash
docker compose up -d          # surrealdb/surrealdb:v4 (default)
# SURREAL_IMAGE=surrealdb/surrealdb:v3.3.0 docker compose up -d
# url ws://127.0.0.1:8000, user/pass root/root
```

## Matrix

| Piece | Declaration |
|---|---|
| Servers | SurrealDB **3.3+** and **4.x** |
| Default image | `surrealdb/surrealdb:v4` |
| JS SDK | `surrealdb@^2.2.2` |
| Package cut | `0.1.0-alpha.1` |
