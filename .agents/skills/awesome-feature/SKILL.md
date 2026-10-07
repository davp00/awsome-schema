---
name: awesome-feature
description: >-
  Plan-first vertical feature delivery for Awesome Schema: hexagonal architecture,
  unit/integration tests with high coverage on new code, Rust e2e and generated-client
  e2e (TypeScript/Rust/Go/etc. when present) when those surfaces change. Use when the
  user says or implies: let's build a feature, lets build a feature, build a feature,
  build feature, new feature, implement a feature, implement feature, ship a feature,
  ship feature, add a feature, create a feature, feature build, feature-build,
  awesome-feature, vertical slice, end-to-end feature, or asks to implement/add a
  capability across DSL, Surreal, codegen, or generated clients.
---

# Awesome Feature

Orchestrate end-to-end feature work. Prefer planning before coding when the change is non-trivial.

**Trigger phrases (non-exhaustive):** "let's build a feature", "build a feature", "build feature", "new feature", "implement a feature", "ship a feature", "add a feature", "create a feature", "feature build", "awesome-feature", "vertical slice".

## Phase 0 — Plan first

If any of these are true, **switch to / stay in Plan mode**, draft a plan, and **wait for user confirmation** before implementing:

- Touches ≥2 crates, or DSL + runtime + codegen
- New SurrealDB live behavior (push/pull/migrate/edges/links)
- New or changed generated-client API
- Unclear acceptance criteria or multiple valid designs

Do **not** edit the plan file during execution. Locked decisions go in the plan; keep scope and out-of-scope explicit.

Simple one-crate bugfixes may skip Plan mode unless the user asks for a plan.

## Phase 1 — Architecture and skills

Follow Split-Core Hexagonal / existing crate boundaries. Load skills as needed:

| Skill / rule | When |
|---|---|
| `split-core-hexagonal-cli` | New CLI commands, ports, use cases, adapters |
| `rust-best-practices` | New or refactored Rust |
| `rust-testing` | Unit/integration tests (prefer TDD) |
| `.cursor/rules/codegen-prefer-reusable.mdc` | Any codegen emitter |
| `git-commit` | Only when the user asks to commit |

**Preferred layer order:** domain/normalize → parser/print (if DSL) → renderer/introspect → migrations/differ → codegen → CLI → tests → docs/roadmap.

Reusable first: shared helpers + data/meta registries; thin per-model/per-language wrappers. No copy-pasted SurrealQL/client bodies.

## Phase 2 — Implement

- Match existing patterns; smallest change that meets acceptance criteria
- No drive-by refactors; no unsolicited markdown docs
- Commit / push / PR only when the user asks
- No Co-authored-by Cursor trailer
- After meaningful decisions/bugfixes: Engram `mem_save` (and session summary when wrapping up)

## Phase 3 — Quality gates

Run what the feature warrants. Skip a gate only when clearly out of scope (state why in the reply).

### Unit / integration

- `cargo test` on touched crates (and workspace excl. e2e when appropriate)
- Prefer TDD for new logic
- Clippy on touched packages: `cargo clippy -p <crate> --all-targets -- -D warnings` when practical

### Coverage (new code)

- For **new** modules/crates/files: aim for **full line coverage on that new code** (llvm-cov / `./scripts/coverage.sh`)
- Do **not** expand scope to chase whole-workspace 100% unless the user asks
- Prefer tests over `coverage(off)` / dead-branch hacks

### Rust e2e (`crates/e2e`)

Run when the feature changes live Surreal/CLI behavior (migrate, `db push`/`pull`, edges, record refs, schema roundtrip):

```bash
cargo test -p e2e --locked
# or a topic binary: cargo test -p e2e --test <topic>
```

Soft-skip without Docker is OK locally; do not treat a soft-skip as proof on CI.

### Generated-client e2e (any language)

Run when the feature changes a **generated client** (types, CRUD, select/projection, helpers, emit paths). Language is not limited to TypeScript:

| Client | Suite (when present) | Trigger examples |
|---|---|---|
| TypeScript | `cd e2e/typescript && npm test` (CI: `typescript-e2e` workflow) | `codegen-typescript`, `createClient`, nested select |
| Rust | `e2e/rust` or documented cargo e2e for the Rust client (when added) | `codegen-rust` client surface |
| Go | `e2e/go` or documented Go client e2e (when added) | Go generator / client |
| Other | Follow `e2e/<lang>/` README or CI workflow for that language | New generator target |

Rules:

- Discover suites under `e2e/` and `.github/workflows/*e2e*` — do not assume only TypeScript exists
- If the feature adds a new generator language, **add** that language’s e2e package/workflow in the same effort when feasible
- Local soft-skip without Docker is OK; CI must hard-fail (`CI=true` / `REQUIRE_SURREAL=1` pattern)
- Prefer hybrid URL-first + testcontainers when adding new client e2e

See [checklist.md](checklist.md) for a copy-paste gate list.

## Phase 4 — Docs and roadmap

When user-facing behavior changes:

- README usage / test instructions if needed
- `docs/roadmap/status.md` and `docs/roadmap/next.md` (mark done / deferred accurately)
- Graphify refresh follow-up commit when the user commits and the post-commit hook dirties `graphify-out/`

## Done definition

A feature is done when:

1. Plan (if required) was confirmed and implemented without silent scope creep
2. Architecture/skills above were respected
3. Unit/integration tests pass; new code meets the coverage bar
4. Required Rust e2e and/or generated-client e2e (for each affected language) were run or explicitly waived
5. Docs/roadmap updated when appropriate
