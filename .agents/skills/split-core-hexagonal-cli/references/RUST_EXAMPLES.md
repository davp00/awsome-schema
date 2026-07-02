# Rust CLI Examples

Complete Rust examples for Split-Core Hexagonal CLI Architecture. See [RUST.md](RUST.md) for patterns and [REFERENCE.md](REFERENCE.md) for layer rules.

---

## Domain entity

```rust
// core/src/domain/models/user.rs
use crate::domain::errors::DomainError;
use crate::domain::models::{Email, Identifier, StringValue};

pub struct User {
    id: Option<Identifier>,
    email: Email,
    first_name: StringValue,
    last_name: StringValue,
}

impl User {
    pub fn new(email: &str, first_name: &str, last_name: &str) -> Result<Self, DomainError> {
        Ok(Self {
            id: None,
            email: Email::new(email)?,
            first_name: StringValue::new(first_name)?,
            last_name: StringValue::new(last_name)?,
        })
    }

    pub fn id(&self) -> Result<Identifier, DomainError> {
        self.id.ok_or(DomainError::IdNotDefined)
    }

    pub fn set_id(&mut self, id: Identifier) {
        self.id = Some(id);
    }
}
```

---

## Use case contract (domain layer)

```rust
// core/src/domain/usecases/import_users.rs
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ImportUsersInput {
    pub file_path: String,
    pub content: Option<String>,
    pub dry_run: bool,
    pub format: ImportFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ImportUsersOutput {
    pub imported: u32,
    pub skipped: u32,
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub enum ImportFormat {
    #[default]
    Csv,
    Json,
}
```

---

## Use case implementation

```rust
// core/src/usecases/import_users.rs
use std::sync::Arc;

use crate::domain::errors::DomainError;
use crate::domain::services::FileSystemPort;
use crate::domain::repositories::UserRepository;
use crate::domain::usecases::import_users::{ImportUsersInput, ImportUsersOutput};
use crate::domain::models::User;

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
            return Ok(ImportUsersOutput {
                imported: 0,
                skipped: 0,
                dry_run: Some(true),
            });
        }

        let raw = match &port.content {
            Some(c) => c.clone(),
            None => self.file_system.read_file(&port.file_path)?,
        };

        let rows = parse_by_format(&raw, port.format)?;
        if rows.is_empty() {
            return Err(DomainError::NoRowsFound);
        }

        let users: Vec<User> = rows
            .into_iter()
            .map(|row| User::new(&row.email, &row.first_name, &row.last_name))
            .collect::<Result<_, _>>()?;

        let saved = self.user_repository.save_many(&users)?;
        Ok(ImportUsersOutput {
            imported: saved.len() as u32,
            skipped: 0,
            dry_run: None,
        })
    }
}
```

---

## Repository port

```rust
// core/src/domain/repositories/user_repository.rs
use std::collections::HashMap;

use crate::domain::errors::DomainError;
use crate::domain::models::User;

pub trait UserRepository: Send + Sync {
    fn find_by_emails(&self, emails: &[String]) -> Result<HashMap<String, User>, DomainError>;
    fn save_many(&self, users: &[User]) -> Result<Vec<User>, DomainError>;
}
```

---

## Infrastructure adapter

```rust
// cli/src/adapters/repositories/user_database_repository.rs
use std::sync::Arc;

use core::domain::errors::DomainError;
use core::domain::models::User;
use core::domain::repositories::UserRepository;

pub struct UserDatabaseRepository {
    pool: Arc<DbPool>,
}

impl UserRepository for UserDatabaseRepository {
    fn save_many(&self, users: &[User]) -> Result<Vec<User>, DomainError> {
        // insert rows, assign ids via set_id on each user
        let _ = self.pool; // …
        if users.is_empty() {
            return Ok(vec![]);
        }
        Err(DomainError::UserNotSaved)
    }

    fn find_by_emails(&self, _emails: &[String]) -> Result<std::collections::HashMap<String, User>, DomainError> {
        Ok(std::collections::HashMap::new())
    }
}
```

---

## I/O ports (domain)

```rust
// core/src/domain/services/io.rs
use crate::domain::errors::DomainError;

pub trait OutputWriter: Send + Sync {
    fn write_line(&self, text: &str) -> Result<(), DomainError>;
}

pub trait InputReader: Send + Sync {
    fn read_all(&self) -> Result<String, DomainError>;
}

pub trait FileSystemPort: Send + Sync {
    fn read_file(&self, path: &str) -> Result<String, DomainError>;
    fn write_file(&self, path: &str, content: &str) -> Result<(), DomainError>;
    fn exists(&self, path: &str) -> Result<bool, DomainError>;
}
```

---

## Command handler

```rust
// cli/src/presentation/commands/users/import.rs
use std::sync::Arc;
use std::path::PathBuf;

use clap::Parser;
use core::domain::services::InputReader;
use core::domain::errors::DomainError;
use core::domain::usecases::import_users::ImportUsersInput;
use core::usecases::import_users::ImportUsersUseCase;

use crate::presentation::commands::GlobalOptions;
use crate::presentation::formatters::{HumanFormatter, JsonFormatter, OutputFormat};
pub struct ImportArgs {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
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
            file_path: args
                .file
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "-".into()),
            dry_run: global.dry_run,
            content: None,
            format: Default::default(),
        };

        if port.file_path == "-" {
            port.content = Some(self.input_reader.read_all()?);
        }

        let result = self.use_case.execute(port)?;
        match global.output {
            OutputFormat::Json => self.json.write_import_result(&result),
            OutputFormat::Human => self.human.write_import_result(&result),
        }
    }
}
```

---

## Exit code map

```rust
// cli/src/presentation/exit_codes.rs
use core::domain::errors::DomainError;

use crate::presentation::error_catalog;

pub fn resolve_exit_code(error: &DomainError) -> i32 {
    match error {
        DomainError::InvalidEmail => 2,
        _ => error_catalog::lookup(error.code()).map(|e| e.exit_code).unwrap_or(1),
    }
}
```

---

## Composition root (di)

```rust
// cli/src/di.rs
use std::sync::Arc;

use core::usecases::import_users::ImportUsersUseCase;

use crate::adapters::io::{LocalFileSystemAdapter, StdioReader, StdioWriter};
use crate::adapters::repositories::UserDatabaseRepository;
use crate::presentation::commands::users::import::ImportUsersCommand;
use crate::presentation::formatters::{HumanFormatter, JsonFormatter};

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
}
```

---

## Test — use case

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockRepo;
    impl UserRepository for MockRepo {
        fn find_by_emails(&self, _: &[String]) -> Result<HashMap<String, User>, DomainError> {
            Ok(HashMap::new())
        }
        fn save_many(&self, _: &[User]) -> Result<Vec<User>, DomainError> {
            Err(DomainError::InternalServerError)
        }
    }

    struct MockFs { content: String }
    impl FileSystemPort for MockFs {
        fn read_file(&self, _: &str) -> Result<String, DomainError> {
            Ok(self.content.clone())
        }
        fn write_file(&self, _: &str, _: &str) -> Result<(), DomainError> { Ok(()) }
        fn exists(&self, _: &str) -> Result<bool, DomainError> { Ok(true) }
    }

    #[test]
    fn dry_run_skips_save() {
        let uc = ImportUsersUseCase::new(Arc::new(MockRepo), Arc::new(MockFs { content: String::new() }));
        let out = uc
            .execute(ImportUsersInput { dry_run: true, ..Default::default() })
            .unwrap();
        assert_eq!(out.imported, 0);
        assert_eq!(out.dry_run, Some(true));
    }

    #[test]
    fn empty_file_returns_no_rows_found() {
        let uc = ImportUsersUseCase::new(Arc::new(MockRepo), Arc::new(MockFs { content: String::new() }));
        let err = uc
            .execute(ImportUsersInput { dry_run: false, ..Default::default() })
            .unwrap_err();
        assert_eq!(err, DomainError::NoRowsFound);
    }
}
```

---

## Test — command handler

```rust
#[test]
fn maps_flags_to_import_users_input() {
    let mock_uc = Arc::new(MockImportUseCase::default());
    let cmd = ImportUsersCommand::new(mock_uc.clone(), /* … */);
    cmd.run(
        ImportArgs { file: Some("/data.csv".into()) },
        &GlobalOptions { dry_run: false, output: OutputFormat::Json },
    )
    .unwrap();
    assert_eq!(mock_uc.last_input().file_path, "/data.csv");
}
```

---

## clap root command tree

```rust
// cli/src/presentation/commands/mod.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "awesome-schema", version, about)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOptions,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Users {
        #[command(subcommand)]
        command: UsersCommands,
    },
}

#[derive(Subcommand)]
pub enum UsersCommands {
    Import(ImportArgs),
}
```
