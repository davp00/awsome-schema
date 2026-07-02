# CLI Architecture Reference

Complete reference for Split-Core Hexagonal CLI Architecture. Self-contained — no external skill dependencies.

## Core philosophy

1. **Deployable unit = CLI application.** One bounded context per CLI unless command namespaces clearly separate domains.
2. **Split the core.** Domain contracts in `core/domain/`; application behavior in `core/usecases/`.
3. **Dependencies point inward.** Infrastructure depends on core. Core never depends on infrastructure.
4. **One action = one use case.** Each operation is one class with one `execute(port)` method.
5. **Ports over frameworks.** Business code depends on interfaces it owns.
6. **Explicit composition.** Single `infrastructure/di` module wires adapters to use cases.
7. **Thin edges, fat use cases.** Command handlers only parse, call `execute`, and format.
8. **Fail with codes, not prose.** Stable `SCREAMING_SNAKE_CASE` error codes in core.
9. **Integrate via adapters.** DB, HTTP, filesystem, and cloud SDKs behind ports.
10. **Test the application core.** Unit-test use cases with mocked ports.

### Unclear boundaries

- **`core/domain/services/`** = outbound **ports** (interfaces): `UserRepository`, `AppLogger`, `FileSystemPort`
- **`core/services/`** = **orchestrators**: wizards, command dispatch — not ports
- **`core/domain/usecases/`** = Input/Output **types only**; **`core/usecases/`** = **classes**
- **Shared package** — extract when two or more deployables need the same integration

---

## Architectural layers

### Layer 1: Domain Core (`core/domain`)

**Responsibility:** Vocabulary and contracts of the bounded context.

**Contains:**
- Entities and value objects (`models/`)
- Repository ports (`repositories/`)
- Outbound service ports (`services/`) — include I/O ports here
- Use-case Input/Output types (`usecases/`)
- Domain event contracts (`events/`)

**May depend on:** Shared value-object libraries, shared domain ports (not adapters), standard library.

**Must never depend on:** CLI frameworks, terminal libraries, `argv`, env, filesystem SDKs, infrastructure.

---

### Layer 2: Application Core (`core/usecases` + `core/services`)

**Responsibility:** Orchestrate business workflows.

**Contains:**
- Use case classes with `execute(port)` (`core/usecases/`)
- Orchestrators (`core/services/`) — wizards, multi-step pipelines, command dispatch

**May depend on:** `core/domain`, shared domain ports.

**Must never depend on:** Adapters, CLI frameworks, env variables, terminal APIs.

**Orchestrator examples:**
- `RunOnboardingWizardService` — prompts across steps, then calls use cases
- `ImportUseCaseHandlerService` — routes by file type to the correct use case

Orchestrators return structured results or throw error codes — they do not write to the terminal.

---

### Layer 3: Infrastructure (`infrastructure/`)

**Contains:**

| Area | Path | Role |
|------|------|------|
| Persistence | `adapters/repositories/` | `{Entity}DatabaseRepository` |
| External APIs | `adapters/services/` | HTTP clients, cloud SDKs, crypto |
| Events | `adapters/events/` | Domain event publisher wrappers |
| Terminal I/O | `adapters/io/` | stdin, stdout, prompts, progress |
| Filesystem | `adapters/io/` or `adapters/services/` | `FileSystemAdapter` |
| Config files | `adapters/config/` | Parse `.toml`/`.yaml`/`.json` |
| Commands | `presentation/commands/` | Subcommand handlers |
| Formatters | `presentation/formatters/` | Human, JSON, YAML output |
| Exit codes | `presentation/exit-codes` | Error code → process exit code |
| Composition | `di` | Wire adapters, use cases, formatters |
| Env constants | `environment` | Env var names and defaults |

**May depend on:** Core, shared packages, frameworks, SDKs.

**Must never depend on:** Handlers calling adapters directly (except `di` and bootstrap).

---

### Layer 4: Shared capability packages (optional)

```
shared-{capability}/
  domain/     → ports, models
  adapters/   → HTTP/SDK implementations
```

Adapters depend on domain only. Compose at CLI `di`, not across packages.

---

## Dependency rules

### Allowed

```
command handler → formatter
command handler → di → use case
use case → domain ports
adapters → domain (implement ports)
shared/adapters → shared/domain
```

### Forbidden

| From | To | Reason |
|------|-----|--------|
| `core/domain` | `infrastructure` | Hexagonal boundary |
| `core/usecases` | `infrastructure/adapters` | Ports only |
| `core/usecases` | terminal/color libraries | Presentation leak |
| `core/usecases` | `process.exit` / `os.Exit` | Exit is bootstrap only |
| `core/domain` | CLI framework arg types | Keep domain agnostic |
| Command handler | repository adapter | Bypass use case |
| Use case A | Use case B directly | Use orchestrator |
| Use case | `argv`, stdin, stdout | Use ports / return DTOs |

### Import discipline

- Use cases import from `../domain` only.
- Handlers import wired instances from `di`, never `new UseCase(...)` inline.
- Only `di` and bootstrap `index` instantiate concrete adapters.

---

## Folder structure

### Single CLI application

```
{cli-name}/
  src/
    index
    core/
      domain/{models,repositories,services,usecases,events}/
      usecases/
      services/                 # optional
    infrastructure/
      di, environment
      adapters/{repositories,services,events,io,config}/
      presentation/
        commands/
          index
          {group}/{verb}.command
        formatters/
        exit-codes
        schemas/                # optional flag validation
    test/
      {useCaseName}.test
      commands/{verb}.command.test
      *.mock
```

### Monorepo placement (Rust)

```
{workspace}/
  Cargo.toml
  crates/
    core/                   # domain + usecases library
    cli/                    # binary + adapters + presentation
    shared-{capability}/    # optional
```

### Bootstrap `index` responsibilities

1. Load `environment` and global flags (`--config`, `--verbose`)
2. Initialize `di`
3. Build command tree
4. Parse `argv` and dispatch handler
5. Success → exit `0`
6. Error → map code, write stderr, exit non-zero
7. `finally` → shutdown (close DB, flush logs)

---

## Naming conventions

### Domain entities

- **Pattern:** `{Concept}` — PascalCase noun (`User`, `Order`)
- **Read projections:** `{Concept}Properties`, `{Concept}View`
- **Not:** `UserEntity`, `UserDTO`

### Value objects

- **Pattern:** `{Concept}Value` or `{PhoneNumber}`
- **Validation errors:** `EMPTY_{FIELD}`, `INVALID_{FIELD}`

### Use cases

- **Class:** `{Verb}{Target}UseCase` — `CreateUserUseCase`, `ImportDataUseCase`
- **Contract file:** `core/domain/usecases/{Name}UseCase` — types only
- **Implementation:** `core/usecases/{Name}UseCase` — class
- **Input/Output:** `{Name}Input`, `{Name}Output`
- **Execute param:** always `port`

### Ports

| Port | Pattern |
|------|---------|
| Repository | `{Entity}Repository` |
| Outbound service | `{Capability}Service` |
| Logger | `AppLogger` |
| ID generator | `IdGeneratorService` |
| Filesystem | `FileSystemPort` |
| Stdout | `OutputWriter` |
| Stderr | `ErrorWriter` |
| Stdin | `InputReader` |
| Interactive | `PromptService` |
| Progress | `ProgressReporter` |
| Events | `EventEmitter` with `publish{Event}` |

### Adapters

| Adapter | Pattern |
|---------|---------|
| Database | `{Entity}DatabaseRepository` |
| HTTP client | `{Name}ApiServiceAdapter` |
| Cloud SDK | `{Provider}{Capability}Service` |
| Logger | `{Provider}AppLogger` |
| Filesystem | `{Provider}FileSystemAdapter` |

### CLI presentation

| Kind | Pattern |
|------|---------|
| CLI app | `{domain}-cli` |
| Command group | plural noun — `users`, `orders` |
| Subcommand | `{verb}` — `create`, `list`, `import` |
| Handler class | `{Verb}{Resource}Command` |
| Handler file | `{verb}.command` |
| Formatter | `{Format}Formatter` |
| Global flags | `--verbose`, `--config`, `--output`, `--dry-run` |

### Errors and tests

- **Errors:** `throw Error('SCREAMING_SNAKE_CASE')`
- **Test file:** `{useCaseName}.test`
- **Mocks:** `repositories.mock`, `services.mock`, `io.mock`

---

## Code organization

| Concern | Location |
|---------|----------|
| Entity invariants | Entity constructor + value objects |
| Business workflow | Use case `execute` |
| Persistence | Repository adapter |
| HTTP / cloud calls | Service adapter |
| Flag definitions | `presentation/commands/` |
| Arg → Input mapping | Command handler or `mappers/` |
| Human/JSON output | `presentation/formatters/` |
| Error codes | Use case / domain |
| Exit codes | `presentation/exit-codes` |
| User-facing error text | Handler / formatter + error catalog |
| Env vars | `infrastructure/environment` only |
| Config merge | Bootstrap (flags → env → file) |
| Prompts | Handler/orchestrator before `execute` |
| Logging | `AppLogger`; level from `--verbose` |

### Use case structure (mandatory)

```
class {Name}UseCase:
  constructor({ deps }: {Name}UseCaseParams):

  execute(port: {Name}Input): Promise<{Name}Output>:
    // 1. validate / load
    // 2. construct or mutate domain objects
    // 3. call ports
    // 4. return output DTO
```

### Entity structure (mutable concepts)

```
class {Entity}:
  private id?: Identifier

  getId(): Identifier:
    if not this.id: throw Error('ID_NOT_DEFINED')

  setId(id): void   // called by repository after insert
```

### Command handler structure (mandatory)

```
class CreateUserCommand:
  async run(args, globalOptions):
    port = mapArgsToInput(args)
    result = await createUserUseCase.execute(port)
    selectFormatter(globalOptions.output).write(result)
```

**Thin handler rule:** parse → optional prompt → `execute` → format. Max ~30 lines; extract mappers or promote logic to use case.

### Use case Input from CLI

Map flags to explicit Input fields — never pass raw `argv`:

```
type ImportUsersUseCaseInput = {
  filePath: string
  dryRun: boolean
  format: 'csv' | 'json'
}
```

---

## Flow of execution

### Standard subcommand

```
User → index (argv) → Command.run → mapArgsToInput → useCase.execute(port)
     → Formatter → stdout → exit 0
```

### Failure

```
useCase throws Error('USER_ALREADY_EXISTS')
  → handler/bootstrap catches
  → exitCodes + error catalog → stderr
  → exit 1
```

### Stdin pipe

```
cat data.csv | app import --format csv
  → InputReader.readAll() in handler
  → build ImportUseCaseInput
  → execute → formatter
```

### Interactive wizard

```
app setup → RunSetupWizardService
  → PromptService collects values
  → builds Input → one or more use cases → formatter
```

---

## Error handling

### Modeling (core)

- Standard error type with **string code** as message
- Codes: `SCREAMING_SNAKE_CASE`, stable, documented

| Layer | Throw |
|-------|-------|
| Value object | `INVALID_EMAIL`, `EMPTY_STRING` |
| Entity getter | `ID_NOT_DEFINED` |
| Use case | `ENTITY_NOT_FOUND`, `{RULE}_VIOLATED` |
| Use case catch | log via `AppLogger`, throw `INTERNAL_SERVER_ERROR` |
| Repository adapter | `ENTITY_NOT_SAVED` |

### CLI mapping (presentation)

| Exit | Meaning |
|------|---------|
| `0` | Success |
| `1` | Business/runtime error |
| `2` | Usage / parser validation |
| `3` | Partial success (optional) |

```
ERROR_CATALOG = {
  USER_ALREADY_EXISTS: { exitCode: 1, message: 'User already exists' },
  INVALID_EMAIL: { exitCode: 2, message: 'Invalid email' },
}
```

- `--output json` errors on stderr: `{ "error": "USER_ALREADY_EXISTS" }`
- No stack traces unless `--verbose`

---

## Validation

| Layer | Responsibility |
|-------|----------------|
| CLI parser | Required flags, types, mutual exclusion → exit `2` |
| Presentation schema | Optional structural validation |
| Use case | Business rules, existence, state |
| Value object | Field invariants at construction |
| Repository adapter | DB constraints as last resort |

---

## Testing

### Use cases (primary)

- Mock all ports: repositories, services, `FileSystemPort`, `PromptService`
- Use real entities and value objects
- Cover happy path, branches, and each error code
- Assert return values and port call counts

### Command handlers

- Mock wired use case and formatters
- Assert `execute` called with correct Input from sample args
- Do not test business rules here

### Formatters and mappers

- Fixed Output → assert rendered string/JSON
- `mapArgsToInput(flags)` → expected Input shape

### Integration

- Repository adapters against test DB; filesystem against temp dir

### E2E (sparse)

- Subprocess: run CLI with args → assert exit code + stdout/stderr

---

## External integrations

- **Filesystem** — always `FileSystemPort` (testable with temp dirs)
- **Database** — connect in `index`, pass to `di`, close on shutdown
- **Remote APIs** — port + HTTP adapter; URL from env or `--api-url`
- **Events** — domain `EventEmitter` port; adapter wraps transport
- **Credentials** — env or config via `environment`; handler may prompt with hidden input, pass token in Input

---

## Configuration

**Precedence:** CLI flags → environment → config file → defaults.

- `infrastructure/environment` — env constants
- `adapters/config/` — file parsing
- Global flags: `--config`, `--verbose`, `--output human|json|yaml`, `--dry-run`
- Resolved config passed via `di` — never read env inside use cases

---

## Observability

- **AppLogger** port; level from `--verbose`
- **stderr** — errors and logs
- **stdout** — command output only (pipe-friendly)
- No logs on stdout when `--output json`
- Optional `app doctor` command as use case for connectivity checks
- Optional `--trace-id` in Input for logger context

---

## Language-agnostic implementation

Conceptual rules above apply to any language. For **Rust** (this skill's primary target), see [RUST.md](RUST.md) and [RUST_EXAMPLES.md](RUST_EXAMPLES.md).

| Concern | Rust approach |
|---------|---------------|
| Arg parsing | `clap` derive in `cli/presentation/commands/` |
| Entry | `main.rs` → bootstrap → clap dispatch |
| Ports | `trait` in `core/domain/`; `Arc<dyn Trait>` in use cases |
| DI | Manual `AppContext` in `cli/src/di.rs` |
| Errors | `DomainError` + `thiserror` in core; exit map in cli |
| Sync/async | Pick one; use `async-trait` if async |
| Distribution | `cargo install` / workspace binary; `core` unchanged |

**Rule:** folder semantics and dependency direction are fixed; Rust uses traits, modules, and `Result` instead of classes and exceptions.
