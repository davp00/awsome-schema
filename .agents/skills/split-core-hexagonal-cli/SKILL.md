---
name: split-core-hexagonal-cli
description: >-
  Design CLI tools with Split-Core Hexagonal Architecture. Use when building
  command-line applications, refactoring monolithic scripts into testable use cases,
  or structuring CLIs with ports, adapters, use cases, formatters, and exit codes.
  Self-contained and language-agnostic. Keywords: cli, command-line, subcommand,
  hexagonal, clean architecture, ports, adapters, use cases, stdin, stdout, exit codes.
license: Proprietary
compatibility: Language-agnostic. Terminal I/O required at presentation layer only.
metadata:
  author: tous-secouristes-services
  version: "1.1.0"
  repository: tous-secouristes-services
  tags: hexagonal,clean-architecture,cli,ports-adapters
---

# Split-Core Hexagonal CLI Architecture

Ports-and-adapters architecture for command-line applications. Business logic lives in use cases; infrastructure implements ports; command handlers are thin presentation.

## When to activate

- New CLI tool or binary entry point
- Refactoring a fat `main` script into layered architecture
- Adding subcommands to a hexagonal project
- Sharing `core/` with other deployables in a monorepo

**Do not use** for throwaway one-liners with no business rules.

## Core philosophy

1. **Deployable unit = CLI application** — one process, one composition root, one command registry
2. **Split the core** — contracts in `core/domain/`; use case classes in `core/usecases/`
3. **Dependencies point inward** — `commands → di → usecases → domain ← adapters`
4. **One action = one use case** — single `execute(port)` per class; param always named `port`
5. **Ports over frameworks** — CLI framework, colors, and terminal APIs live at the edge only
6. **Explicit composition** — `infrastructure/di` wires everything; no service locator
7. **Subcommand = use case** — `app users create` → `CreateUserUseCase.execute(port)`
8. **No print in use cases** — I/O via ports; formatters render output
9. **Fail with codes** — `SCREAMING_SNAKE_CASE` in core; human messages and exit codes at presentation
10. **Test use cases** — mock ports; do not test business rules through subprocess only

## Split core (non-negotiable)

| Location | Contains |
|----------|----------|
| `core/domain/usecases/` | `{Name}UseCaseInput` / `Output` types **only** |
| `core/usecases/` | `{Name}UseCase` class with `execute(port)` |
| `core/domain/repositories/` | `{Entity}Repository` port interfaces |
| `core/domain/services/` | Outbound port interfaces (`AppLogger`, `FileSystemPort`, …) |
| `core/services/` | Orchestrators only (wizards, command dispatch) — optional |
| `infrastructure/adapters/` | `{Entity}DatabaseRepository`, HTTP/SDK adapters |
| `infrastructure/di` | Composition root — only place that instantiates adapters |
| `infrastructure/presentation/commands/` | Subcommand handlers |

**Boundaries:**

- `core/domain/services/` = outbound **ports** (interfaces)
- `core/services/` = **orchestrators** (coordinate use cases, build Input from prompts)
- Never put implementation logic in `core/domain/usecases/` files

## Folder layout

```
{cli-name}/
  src/
    index                       # bootstrap: argv, di, exit
    core/domain/{models,repositories,services,usecases,events}/
    core/usecases/
    core/services/              # optional
    infrastructure/
      di, environment
      adapters/{repositories,services,io,config}/
      presentation/
        commands/
        formatters/
        exit-codes
  test/
```

## Dependency diagram

```
presentation/commands → formatters
        ↓
   infrastructure/di
        ↓
   core/usecases
        ↓
   core/domain  ←  infrastructure/adapters
```

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Business/runtime error |
| `2` | Usage / invalid arguments |

## Progressive disclosure — read before implementing

1. [references/REFERENCE.md](references/REFERENCE.md) — layers, naming, flow, errors, validation, testing, integrations
2. [references/EXAMPLES.md](references/EXAMPLES.md) — entity, use case, port, adapter, handler, formatter, tests
3. [references/CHECKLIST.md](references/CHECKLIST.md) — verification, anti-patterns, migration

## Quick naming reference

| Kind | Pattern |
|------|---------|
| Use case | `{Verb}{Entity}UseCase` |
| Repository port | `{Entity}Repository` |
| Repository adapter | `{Entity}DatabaseRepository` |
| Service port | `{Capability}Service` |
| Command handler | `{Verb}{Resource}Command` |
| Handler file | `{verb}.command` |
| Formatter | `{Format}Formatter` |
| I/O ports | `OutputWriter`, `InputReader`, `PromptService` |
| CLI app | `{domain}-cli` |
