use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "awesome-schema",
    version,
    about = "Schema modeling, migration, and code generation for SurrealDB and beyond."
)]
pub struct Cli {
    #[arg(long, default_value = "awesome.schema", global = true)]
    pub schema: String,

    #[arg(long, default_value = "migrations", global = true)]
    pub migrations_dir: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new Awesome Schema project.
    Init,
    /// Validate the schema file.
    Validate,
    /// Format the schema file.
    Format {
        #[arg(long)]
        write: bool,
    },
    /// Generate schema SQL or client models.
    Generate {
        #[arg(long, value_enum, default_value_t = GenerateTarget::Schema)]
        target: GenerateTarget,
    },
    /// Migration commands.
    Migrate {
        #[command(subcommand)]
        command: MigrateCommands,
    },
    /// Database introspection and push commands.
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GenerateTarget {
    Schema,
    Rust,
    Typescript,
}

#[derive(Debug, Subcommand)]
pub enum MigrateCommands {
    /// Diff the schema and create a migration in development mode.
    Dev {
        #[arg(long)]
        name: Option<String>,
    },
    /// Create an empty migration directory.
    Create { name: String },
    /// Show migration status.
    Status,
    /// Apply pending migrations to the database.
    Apply,
}

#[derive(Debug, Subcommand)]
pub enum DbCommands {
    /// Pull schema from the database.
    Pull,
    /// Push schema to the database.
    Push,
}
