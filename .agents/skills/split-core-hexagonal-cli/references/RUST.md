# Rust Implementation Guide

Rust-specific patterns for Split-Core Hexagonal CLI Architecture. Conceptual layers match [REFERENCE.md](REFERENCE.md); this document covers crates, modules, traits, errors, and tooling.

---

## Cargo workspace layout

### Recommended monorepo

```
{workspace}/
  Cargo.toml                    # workspace members
  crates/
    core/                       # domain + use cases (library, no CLI deps)
    cli/                        # binary: di, adapters, presentation
    {capability}/               # optional shared crates (parser, sql, …)
```

| Crate | Contains | May depend on |
|-------|----------|---------------|
| `core` | `domain/`, `usecases/`, `services/` | `std` only; optional pure libs (`serde`, `thiserror`) |
| `cli` | `di`, `adapters/`, `presentation/` | `core`, `clap`, `tokio`, DB/SDK crates |
| `{capability}` | shared ports + adapters when 2+ binaries need them | domain of that capability only |

**Rule:** `core` must not depend on `cli`. Other crates may depend on `core` but never on `cli`.

### Single-crate CLI (small tools)

One crate with the same **module tree** as below. Split into `core` + `cli` when reuse or compile-time isolation is needed.

---

## Module tree (maps to folders)

### `core` crate

```
src/
  lib.rs
  domain/
    mod.rs
    models/
    repositories/       # trait UserRepository { … }
    services/           # trait FileSystemPort { … }
    usecases/           # ImportUsersInput, ImportUsersOutput (types only)
    errors.rs           # DomainError, error codes
  usecases/
    mod.rs
    import_users.rs     # ImportUsersUseCase
  services/             # optional orchestrators
```

### `cli` crate

```
src/
  main.rs               # bootstrap: parse → di → dispatch → exit
  lib.rs                # optional: re-export for integration tests
  di.rs                 # composition root
  environment.rs
  adapters/
    mod.rs
    repositories/
    services/
    io/
    config/
  presentation/
    mod.rs
    commands/
    formatters/
    exit_codes.rs
    error_catalog.rs
```

Register modules in `mod.rs` files; **folder semantics are fixed** even when Rust flattens them into modules.

---

## Recommended dependencies

| Crate | Used in | Purpose |
|-------|---------|---------|
| `clap` | `cli` | Subcommands, global flags |
| `thiserror` | `core` | `DomainError` enum |
| `anyhow` | `cli` (`main`, `di`) | Unexpected infra failures at boundary |
| `serde` / `serde_json` | `core` (types), `cli` (formatters) | JSON output |
| `mockall` | `core` dev-deps | Port trait mocks |
| `tokio` | `cli` (optional) | Async runtime when ports are async |
| `async-trait` | `core` + `cli` (optional) | Object-safe async port traits |

Pin versions in workspace `[workspace.dependencies]` and inherit in member crates.

---

## Ports: traits in domain

Repository and service ports are **traits** owned by `core`. Adapters in `cli` implement them.

```rust
// core/src/domain/repositories/user_repository.rs
use crate::domain::models::User;
use crate::domain::errors::DomainError;

pub trait UserRepository: Send + Sync {
    fn find_by_emails(
        &self,
        emails: &[String],
    ) -> Result<std::collections::HashMap<String, User>, DomainError>;

    fn save_many(&self, users: &[User]) -> Result<Vec<User>, DomainError>;
}
```

**Guidelines:**

- Put traits in `domain/repositories/` or `domain/services/`, not in `usecases/`.
- Add `Send + Sync` when use cases run behind `Arc` or async runtimes.
- Prefer `&self` on trait methods; use cases hold `Arc<dyn UserRepository>`.
- Object-safe traits only (`dyn Trait` in DI). No generic methods on port traits unless using `async-trait` consistently.
- I/O ports (`OutputWriter`, `InputReader`, `PromptService`, `FileSystemPort`) live in `domain/services/`.

---

## Use cases: struct + execute

One struct per use case. Constructor takes port dependencies; **`execute` takes Input only** (the `port` parameter name is mandatory).

```rust
// core/src/usecases/import_users.rs
pub struct ImportUsersUseCase {
    user_repository: Arc<dyn UserRepository>,
    file_system: Arc<dyn FileSystemPort>,
}

impl ImportUsersUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        file_system: Arc<dyn FileSystemPort>,
    ) -> Self {
        Self { user_repository, file_system }
    }

    pub fn execute(&self, port: ImportUsersInput) -> Result<ImportUsersOutput, DomainError> {
        if port.dry_run {
            return Ok(ImportUsersOutput { imported: 0, skipped: 0, dry_run: Some(true) });
        }
        // validate → domain objects → call ports → return Output
    }
}
```

**Sync vs async:**

| Style | When | Port signature |
|-------|------|----------------|
| Sync | File/CLI tools, blocking DB | `fn execute(&self, port: Input) -> Result<Output, DomainError>` |
| Async | HTTP-heavy, concurrent I/O | `async fn execute(&self, port: Input) -> Result<Output, DomainError>` + `async-trait` on ports |

Pick one style per codebase; do not mix sync use cases with async ports.

Use cases **never** import `clap`, `std::process`, `std::io` (stdio), or adapter types.

---

## Errors: typed codes in core

Use a single error enum in `core` with stable string codes for presentation mapping.

```rust
// core/src/domain/errors.rs
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("USER_ALREADY_EXISTS")]
    UserAlreadyExists,
    #[error("NO_ROWS_FOUND")]
    NoRowsFound,
    #[error("INVALID_EMAIL")]
    InvalidEmail,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError,
}

impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UserAlreadyExists => "USER_ALREADY_EXISTS",
            Self::NoRowsFound => "NO_ROWS_FOUND",
            Self::InvalidEmail => "INVALID_EMAIL",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
        }
    }
}
```

- Variant names: `PascalCase`; **`code()` returns `SCREAMING_SNAKE_CASE`** for catalogs and JSON errors.
- Presentation maps `DomainError` → exit code + human message in `presentation/exit_codes.rs` and `error_catalog.rs`.
- `clap` parse errors → exit `2` in `main`, not via `DomainError`.
- Optional: `anyhow` only at the **cli boundary** (`main`) for unexpected infrastructure failures; business rules use `DomainError`.

---

## Dependency injection

Manual wiring in `cli/src/di.rs`. No service locator, no global `OnceLock` for repositories.

```rust
// cli/src/di.rs
pub struct AppContext {
    pub import_users: ImportUsersCommand,
}

impl AppContext {
    pub fn build(config: &AppConfig) -> Result<Self, anyhow::Error> {
        let user_repository = Arc::new(UserDatabaseRepository::new(&config.database_url)?);
        let file_system = Arc::new(LocalFileSystemAdapter);
        let import_use_case = Arc::new(ImportUsersUseCase::new(user_repository, file_system));
        let stdout = Arc::new(StdioWriter::stdout());
        Ok(Self {
            import_users: ImportUsersCommand::new(
                import_use_case,
                Arc::new(StdioReader),
                Arc::new(JsonFormatter::new(stdout.clone())),
                Arc::new(HumanFormatter::new(stdout)),
            ),
        })
    }

    pub async fn shutdown(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
```

- Pass `Arc<dyn Trait>` into use cases and commands.
- Handlers receive wired commands from `AppContext`, never construct use cases inline.
- `AppConfig` is built in bootstrap from flags → env → file (see [REFERENCE.md](REFERENCE.md#configuration)).

---

## Presentation: clap + thin handlers

**clap** (derive) lives only under `presentation/commands/`. Map parsed args to Input types; never pass `clap` types into `core`.

```rust
// cli/src/presentation/commands/users/import.rs
#[derive(Parser)]
pub struct ImportArgs {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    #[arg(long, default_value = "csv")]
    pub format: ImportFormat,
}

pub struct ImportUsersCommand {
    use_case: Arc<ImportUsersUseCase>,
    input_reader: Arc<dyn InputReader>,
    json: Arc<JsonFormatter>,
    human: Arc<HumanFormatter>,
}

impl ImportUsersCommand {
    pub fn run(&self, args: ImportArgs, global: &GlobalOptions) -> Result<(), DomainError> {
        let mut port = ImportUsersInput {
            file_path: args.file.map(|p| p.display().to_string()).unwrap_or_else(|| "-".into()),
            dry_run: global.dry_run,
            format: args.format.into(),
            content: None,
        };
        if port.file_path == "-" {
            port.content = Some(self.input_reader.read_all()?);
        }
        let result = self.use_case.execute(port)?;
        let formatter = if global.output == OutputFormat::Json { &*self.json } else { &*self.human };
        formatter.write_import_result(&result)?;
        Ok(())
    }
}
```

Global flags (`--verbose`, `--config`, `--output`, `--dry-run`) on a root `Cli` struct; subcommands nest underneath.

---

## Bootstrap `main.rs`

Only `main` (or a thin `run()` it calls) may call `std::process::exit`.

```rust
// cli/src/main.rs
use clap::Parser;
use cli::di::AppContext;
use cli::presentation::commands::Cli;
use cli::presentation::exit_codes::resolve_exit_code;
use cli::presentation::error_catalog::write_error;

#[tokio::main]  // omit if fully sync
async fn main() {
    let cli = Cli::parse();
    let config = match cli.load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    let ctx = match AppContext::build(&config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let result = async {
        match cli.command {
            Commands::Users { command } => match command {
                UsersCommands::Import(args) => ctx.import_users.run(args, &cli.global),
            },
        }
    }
    .await;

    match result {
        Ok(()) => {
            let _ = ctx.shutdown().await;
            std::process::exit(0);
        }
        Err(e) => {
            write_error(&mut std::io::stderr(), &e, &cli.global);
            let _ = ctx.shutdown().await;
            std::process::exit(resolve_exit_code(&e));
        }
    }
}
```

---

## Formatters

Formatters depend on `OutputWriter` port, not raw `stdout`:

```rust
pub trait OutputWriter: Send + Sync {
    fn write_line(&self, text: &str) -> std::io::Result<()>;
}

pub struct JsonFormatter {
    out: Arc<dyn OutputWriter>,
}

impl JsonFormatter {
    pub fn write_import_result(&self, result: &ImportUsersOutput) -> Result<(), DomainError> {
        let json = serde_json::to_string(result).map_err(|_| DomainError::InternalServerError)?;
        self.out.write_line(&json).map_err(|_| DomainError::InternalServerError)?;
        Ok(())
    }
}
```

`--output json`: valid JSON on stdout only; logs and errors on stderr.

---

## Testing

### Use cases (`core` crate)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    struct MockUserRepository { save_calls: RefCell<Vec<()>> }
    impl UserRepository for MockUserRepository { /* … */ }

    #[test]
    fn dry_run_skips_save() {
        let repo = Arc::new(MockUserRepository::default());
        let fs = Arc::new(MockFileSystem::default());
        let uc = ImportUsersUseCase::new(repo.clone(), fs);
        let out = uc.execute(ImportUsersInput { dry_run: true, ..Default::default() }).unwrap();
        assert_eq!(out.imported, 0);
        assert!(repo.save_calls().is_empty());
    }
}
```

- Manual test doubles or **`mockall`** for port traits.
- Test every `DomainError` variant on critical paths.
- Keep entity/value-object tests in `core`; no subprocess.

### Commands (`cli` crate)

- Unit-test `run()` with mock use case + formatters.
- Assert `execute` receives correct Input from sample `ImportArgs`.
- Do not re-test business rules here.

### E2E (sparse)

- `assert_cmd` or `std::process::Command` against built binary.
- Assert exit code + stdout/stderr shape only.

---

## Cargo.toml patterns

```toml
# crates/core/Cargo.toml
[dependencies]
thiserror = { workspace = true }
serde = { workspace = true, features = ["derive"] }

[dev-dependencies]
mockall = { workspace = true }

# crates/cli/Cargo.toml
[[bin]]
name = "awesome-schema"
path = "src/main.rs"

[dependencies]
core = { path = "../core" }
clap = { workspace = true, features = ["derive"] }
anyhow = { workspace = true }
tokio = { workspace = true, features = ["rt-multi-thread", "macros"] }
```

Use `cli/src/lib.rs` when integration tests need to import presentation modules without running `main`.

---

## Rust-specific anti-patterns

| Anti-pattern | Fix |
|--------------|-----|
| `unwrap()` in use case | Return `DomainError` |
| `println!` in `core` | Formatter + `OutputWriter` in `cli` |
| `clap::Parser` in `core` | Map args in command handler |
| Concrete adapter type in use case | `Arc<dyn Port>` |
| `lazy_static` / global DB pool | Wire in `AppContext::build` |
| `Box<dyn Error>` in use case | `Result<_, DomainError>` |
| Business logic in `match` on subcommands in `main` | Delegate to `{Verb}Command::run` |

See [CHECKLIST.md](CHECKLIST.md) for the full list and [RUST_EXAMPLES.md](RUST_EXAMPLES.md) for end-to-end code.
