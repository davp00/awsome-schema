# CLI Architecture Checklist

Verification checklist, anti-patterns, and migration guide.

## Checklist

### Split core

- [ ] `core/domain` and `core/usecases` physically separated
- [ ] Use-case Input/Output types in `core/domain/usecases/` only
- [ ] Use case classes in `core/usecases/` with `execute(port)`
- [ ] Repository ports in `core/domain/repositories/`
- [ ] Outbound ports in `core/domain/services/`
- [ ] No framework or CLI imports in `core/`

### Structure

- [ ] `infrastructure/di` is the only composition root
- [ ] Commands in `presentation/commands/`
- [ ] Formatters in `presentation/formatters/`
- [ ] I/O adapters in `adapters/io/`
- [ ] Env vars read only in `environment` module

### Dependencies

- [ ] Use cases depend on port interfaces, not adapters
- [ ] Use cases do not write to stdout/stderr or read `argv`
- [ ] Handlers do not call repositories directly
- [ ] Only bootstrap calls `exit`
- [ ] Presentation imports wired instances from `di`

### Naming

- [ ] Use cases: `{Verb}{Entity}UseCase`
- [ ] Repository adapters: `{Entity}DatabaseRepository`
- [ ] Execute parameter named `port`
- [ ] Error codes: `SCREAMING_SNAKE_CASE`

### Commands

- [ ] One subcommand → one use case (or documented orchestrator)
- [ ] Handlers only parse, execute, format (≤30 lines)
- [ ] `--output json` produces valid JSON on stdout only
- [ ] `--help` documents flags per command

### Errors and UX

- [ ] Exit code map in one module
- [ ] Usage/parser errors exit `2`
- [ ] Results on stdout, errors on stderr
- [ ] Pipe-friendly (no decorative stdout in json mode)

### Testing

- [ ] Unit test every use case with mocked ports
- [ ] Happy path and primary error paths covered
- [ ] Handler tests for flag → Input mapping
- [ ] Formatter tests for output shape

---

## Anti-patterns

1. **Fat `main`** — parsing args and business logic in one file
2. **`print` in use case** — breaks formatters and tests
3. **Business logic in formatter** — formatters only render
4. **Use case reads `argv` or stdin** — breaks reuse and testability
5. **Global mutable config** — pass resolved config via `di`
6. **Mixed JSON and log lines on stdout** — breaks piping
7. **Inconsistent JSON error shape** — unify error envelope
8. **Skipping use case for "simple" commands** — use a thin query use case
9. **Prompts inside use case** — handler/orchestrator builds Input
10. **`process.exit` in use case** — only bootstrap exits
11. **God command** — unrelated operations in one subcommand
12. **Subprocess-only tests** — primary tests target use cases
13. **Use case imports concrete adapter** — ports only
14. **Implementation in `core/domain/usecases/`** — types only there
15. **Handler instantiates `new UseCase(...)`** — use `di`

---

## Migration guide

### From a monolithic script

1. List verbs → subcommands
2. Define `{Verb}UseCaseInput` / `Output` per command
3. Move logic to `core/usecases/{Verb}UseCase`
4. Extract ports for DB, HTTP, filesystem
5. Implement adapters in `infrastructure/adapters/`
6. Create `di` wiring
7. Thin handlers: parse → `execute` → format
8. Move `console.log` formatting to formatters
9. Centralize exit codes
10. Add use case tests, then handler tests

### From an existing service codebase

1. Reuse or copy `core/domain` and `core/usecases`
2. Replace HTTP/GraphQL presentation with `presentation/commands/`
3. Add `adapters/io/` and formatters
4. Reuse repository/service adapters in `di`
5. Replace server `index` with CLI bootstrap `index`

### Incremental strangle

```
legacy main
  → extract one command at a time to handler + use case
  → legacy dispatches unknown commands to old code
  → delete legacy when all commands migrated
```

### Per-command order

```
use case types → use case + tests → ports → adapters → di → handler → delete old code
```

---

## Summary

Split-Core Hexagonal CLI Architecture keeps business logic in testable use cases behind ports. Commands, formatters, exit codes, and terminal I/O are strictly presentation. Dependencies point inward. One subcommand maps to one use case. stdout carries results; stderr carries errors.
