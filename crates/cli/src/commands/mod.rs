pub mod db;
pub mod format;
mod generate;
pub mod init;
pub mod migrate;
mod validate;

use anyhow::Result;

use crate::app::{Cli, Commands, DbCommands, MigrateCommands};
use crate::di::AppContext;
use crate::output::Printer;

pub fn dispatch(cli: &Cli, printer: &Printer) -> Result<()> {
    let context = AppContext::with_paths(cli.schema.clone(), cli.migrations_dir.clone())?;

    match &cli.command {
        Commands::Init => init::run(&context, printer),
        Commands::Validate => validate::run(&context, printer),
        Commands::Format { write } => format::run(&context, *write, printer),
        Commands::Generate { target } => generate::run(&context, *target, printer),
        Commands::Migrate { command } => match command {
            MigrateCommands::Dev { name } => migrate::run_dev(&context, name.clone(), printer),
            MigrateCommands::Create { name } => migrate::run_create(&context, name, printer),
            MigrateCommands::Status => migrate::run_status(&context, printer),
            MigrateCommands::Apply => migrate::run_apply(&context, printer),
            MigrateCommands::Rollback { steps } => migrate::run_rollback(&context, *steps, printer),
        },
        Commands::Db { command } => match command {
            DbCommands::Pull { split_by_table, force } => db::run_pull(&context, *split_by_table, *force, printer),
            DbCommands::Push => db::run_push(&context, printer),
        },
    }
}
