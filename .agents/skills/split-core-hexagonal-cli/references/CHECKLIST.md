# CLI Architecture Checklist

Verification checklist, anti-patterns, and migration guide.

## Checklist

### Split core

- [ ] `core` and `cli` are separate workspace crates (or separate module trees)
- [ ] Use-case Input/Output types in `core/src/domain/usecases/` only
- [ ] Use case structs in `core/src/usecases/` with `execute(&self, port)`
- [ ] Repository ports are traits in `core/src/domain/repositories/`
- [ ] Outbound ports are traits in `core/src/domain/services/`
- [ ] `DomainError` defined in `core`; no `clap` or `std::process` in `core`

### Structure

- [ ] `cli/src/di.rs` (`AppContext`) is the only composition root
- [ ] Commands in `cli/src/presentation/commands/`
- [ ] Formatters in `cli/src/presentation/formatters/`
- [ ] I/O adapters in `cli/src/adapters/io/`
- [ ] Env vars read only in `environment.rs`

### Dependencies

- [ ] Use cases depend on `Arc<dyn Port>`, not concrete adapters
- [ ] Use cases do not use `println!`, `eprintln!`, or read `std::env`
- [ ] Handlers do not call repository adapters directly
- [ ] Only `main.rs` calls `std::process::exit`
- [ ] Commands receive wired deps from `AppContext`

### Naming

- [ ] Use cases: `{Verb}{Entity}UseCase`
- [ ] Repository adapters: `{Entity}DatabaseRepository`
- [ ] Execute parameter named `port`
- [ ] `DomainError::code()` returns `SCREAMING_SNAKE_CASE`

### Commands

- [ ] One subcommand → one use case (or documented orchestrator)
- [ ] Handlers only parse (clap), execute, format (≤30 lines)
- [ ] `--output json` produces valid JSON on stdout only
- [ ] `clap` `--help` documents flags per command

### Errors and UX

- [ ] Exit code map in `presentation/exit_codes.rs`
- [ ] clap / usage errors exit `2`
- [ ] Results on stdout, errors on stderr
- [ ] Pipe-friendly (no decorative stdout in json mode)

### Testing

- [ ] Unit test every use case in `core` with mocked port traits
- [ ] Happy path and primary `DomainError` paths covered
- [ ] Handler tests for flag → Input mapping
- [ ] Formatter tests for output shape
- [ ] `mockall` or manual mocks in `core` dev-dependencies

---

## Anti-patterns

1. **Fat `main.rs`** — clap parsing and business logic in one file
2. **`println!` in use case** — breaks formatters and tests
3. **Business logic in formatter** — formatters only render
4. **Use case reads stdin or env** — breaks reuse and testability
5. **Global `OnceLock` / `lazy_static` for repos** — pass via `AppContext`
6. **Mixed JSON and log lines on stdout** — breaks piping
7. **Inconsistent JSON error shape** — unify error envelope
8. **Skipping use case for "simple" commands** — use a thin query use case
9. **Prompts inside use case** — handler/orchestrator builds Input
10. **`std::process::exit` outside `main`** — only bootstrap exits
11. **God command** — unrelated operations in one subcommand
12. **Subprocess-only tests** — primary tests target use cases in `core`
13. **Use case imports concrete adapter** — `Arc<dyn Trait>` only
14. **Logic in `core/domain/usecases/`** — types only there
15. **Handler constructs use case inline** — use `AppContext`
16. **`unwrap()` in use case** — return `DomainError`
17. **`clap::Parser` in `core`** — map args in presentation layer

---

## Migration guide

### From a monolithic `main.rs`

1. List verbs → clap subcommands
2. Define `{Verb}Input` / `{Verb}Output` per command in `core`
3. Move logic to `core/src/usecases/{verb}.rs`
4. Extract port traits for DB, HTTP, filesystem
5. Implement adapters in `cli/src/adapters/`
6. Create `AppContext` in `cli/src/di.rs`
7. Thin handlers: parse → `execute` → format
8. Move `println!` formatting to formatters
9. Centralize exit codes in `presentation/exit_codes.rs`
10. Add use case tests in `core`, then handler tests in `cli`

### From an existing service codebase

1. Reuse or extract `crates/core` (domain + usecases)
2. Replace HTTP/GraphQL handlers with `cli/presentation/commands/`
3. Add `cli/adapters/io/` and formatters
4. Reuse repository/service adapters in `AppContext::build`
5. Replace server entry with `cli/src/main.rs` bootstrap

### Incremental strangle

```
legacy main
  → extract one command at a time to handler + use case
  → legacy dispatches unknown commands to old code
  → delete legacy when all commands migrated
```

### Per-command order

```
Input/Output types → use case + #[test] in core → port traits → adapters in cli → di → command handler → delete old code
```

---

## Summary

Split-Core Hexagonal CLI Architecture keeps business logic in testable use cases behind ports. Commands, formatters, exit codes, and terminal I/O are strictly presentation. Dependencies point inward. One subcommand maps to one use case. stdout carries results; stderr carries errors.
