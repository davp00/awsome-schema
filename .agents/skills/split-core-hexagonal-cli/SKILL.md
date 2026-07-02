---
name: split-core-hexagonal-cli
description: >-
  Design Rust CLI tools with Split-Core Hexagonal Architecture. Use when building
  Cargo workspace CLIs, structuring crates/cli + crates/core, defining port traits,
  use cases with execute(port), clap handlers, formatters, DomainError, and exit codes.
  Keywords: rust, cli, cargo, clap, hexagonal, ports, adapters, use cases, thiserror.
license: Proprietary
compatibility: Rust (edition 2024). Terminal I/O at presentation layer only.
metadata:
  author: tous-secouristes-services
  version: "1.2.0"
  repository: tous-secouristes-services
  tags: rust,hexagonal,clean-architecture,cli,ports-adapters,clap
---

# Split-Core Hexagonal CLI Architecture (Rust)

Ports-and-adapters architecture for Rust CLI applications. Business logic lives in the `core` crate; the `cli` crate implements port traits and thin clap handlers.

**Default stack:** Cargo workspace, `clap` (derive), `thiserror` for `DomainError`, manual `di` wiring with `Arc<dyn Port>`.

## When to activate

- New CLI binary or `crates/cli` entry point
- Refactoring a fat `main.rs` into layered architecture
- Adding clap subcommands to a hexagonal workspace
- Sharing `crates/core` with other workspace members (codegen, tests, future binaries)

**Do not use** for throwaway one-liners with no business rules.

## Core philosophy

1. **Deployable unit = `cli` binary** — one process, one `di.rs`, one clap command tree
2. **Split the core** — `crates/core`: contracts in `domain/`; use case structs in `usecases/`
3. **Dependencies point inward** — `cli` → `core`; `core` never depends on `cli`
4. **One action = one use case** — single `execute(&self, port)`; parameter always named `port`
5. **Ports = traits** — `dyn Trait` in domain; `clap`/stdio/DB only in `cli`
6. **Explicit composition** — `cli/src/di.rs` builds `AppContext`; no global service locator
7. **Subcommand = use case** — `app users create` → `CreateUserUseCase::execute(port)`
8. **No println in use cases** — I/O via port traits; formatters write stdout
9. **Fail with codes** — `DomainError::code()` returns `SCREAMING_SNAKE_CASE`; exit mapping in presentation
10. **Test use cases in `core`** — manual mocks or `mockall`; subprocess tests are sparse E2E only

## Split core (non-negotiable)

| Location | Contains |
|----------|----------|
| `core/src/domain/usecases/` | `{Name}Input` / `{Name}Output` types **only** |
| `core/src/usecases/` | `{Name}UseCase` struct with `execute(&self, port)` |
| `core/src/domain/repositories/` | `{Entity}Repository` traits |
| `core/src/domain/services/` | Outbound port traits (`AppLogger`, `FileSystemPort`, …) |
| `core/src/domain/errors.rs` | `DomainError` enum + `code()` |
| `core/src/services/` | Orchestrators only — optional |
| `cli/src/adapters/` | `{Entity}DatabaseRepository`, I/O, HTTP/SDK impls |
| `cli/src/di.rs` | `AppContext` — only place that constructs adapters |
| `cli/src/presentation/commands/` | clap structs + `{Verb}{Resource}Command` |

**Boundaries:**

- `core/domain/services/` = outbound **ports** (traits)
- `core/services/` = **orchestrators** (coordinate use cases, build Input from prompts)
- Never put use case logic in `core/domain/usecases/` files (types only)

## Cargo workspace layout

```
{workspace}/
  Cargo.toml
  crates/
    core/src/
      domain/{models,repositories,services,usecases,errors}.rs
      usecases/
      services/                 # optional
    cli/src/
      main.rs                   # bootstrap: clap → di → exit
      di.rs                     # AppContext
      environment.rs
      adapters/{repositories,services,io,config}/
      presentation/
        commands/
        formatters/
        exit_codes.rs
        error_catalog.rs
    {capability}/               # optional shared crates
```

## Dependency diagram

```
cli/presentation/commands → formatters
        ↓
   cli/di (AppContext)
        ↓
   core/usecases
        ↓
   core/domain  ←  cli/adapters (impl traits)
```

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Business/runtime error (`DomainError`) |
| `2` | Usage / clap validation |

## Progressive disclosure — read before implementing

1. [references/RUST.md](references/RUST.md) — **start here**: traits, errors, clap, di, testing, dependencies
2. [references/REFERENCE.md](references/REFERENCE.md) — layers, naming, flow, validation (language-neutral concepts)
3. [references/RUST_EXAMPLES.md](references/RUST_EXAMPLES.md) — full Rust examples
4. [references/CHECKLIST.md](references/CHECKLIST.md) — verification, anti-patterns, migration

## Quick naming reference

| Kind | Pattern |
|------|---------|
| Use case struct | `{Verb}{Entity}UseCase` |
| Input / Output | `{Verb}{Entity}Input`, `{Verb}{Entity}Output` |
| Repository port | `trait {Entity}Repository` |
| Repository adapter | `{Entity}DatabaseRepository` |
| Service port | `trait {Capability}Port` or `{Capability}Service` |
| Command struct | `{Verb}{Resource}Command` |
| clap args module | `presentation/commands/{group}/{verb}.rs` |
| Formatter | `{Format}Formatter` |
| Composition root | `AppContext` in `di.rs` |
| Error enum | `DomainError` with `code() -> &'static str` |
| Binary crate | `cli` (or `{domain}-cli`) |
