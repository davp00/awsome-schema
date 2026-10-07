# Awesome Feature — gate checklist

Copy into the plan or PR notes; check what applies.

## Plan

- [ ] Plan mode used if multi-crate / DSL / live DB / generated client
- [ ] Acceptance criteria + out of scope locked
- [ ] User confirmed plan before implementation

## Architecture

- [ ] Hexagonal / crate boundaries respected
- [ ] Loaded `split-core-hexagonal-cli` / `rust-best-practices` / `rust-testing` / codegen reusable rule as needed
- [ ] Shared helpers first; no per-entity SurrealQL/client clones

## Unit / coverage

- [ ] `cargo test` on touched crates
- [ ] Clippy clean on touched packages (when practical)
- [ ] New modules/files: aim full llvm-cov on **new** code
- [ ] Did not expand to whole-workspace 100% chase

## Rust e2e

- [ ] N/A — no live Surreal/CLI behavior change
- [ ] `cargo test -p e2e` (or topic binary) when needed

## Generated-client e2e

- [ ] N/A — no generated client surface change
- [ ] TypeScript: `cd e2e/typescript && npm test` (if TS client affected)
- [ ] Rust client e2e (if present and affected)
- [ ] Go client e2e (if present and affected)
- [ ] Other `e2e/<lang>/` suite for any new/changed generator

## Docs / memory / git

- [ ] README / roadmap updated if user-facing
- [ ] Engram save for decisions/bugs
- [ ] Commit / push / graphify only if user asked
