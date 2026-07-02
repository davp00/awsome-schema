use anyhow::Result;

use schema_core::{MigrateApplyInput, MigrateCreateInput, MigrateDevInput, MigrateStatusInput};

use crate::di::AppContext;
use crate::output::Printer;

pub fn run_dev(context: &AppContext, name: Option<String>, printer: &Printer) -> Result<()> {
    let output = context.migrate_dev.execute(MigrateDevInput {
        migrations_dir: context.migrations_dir.clone(),
        migration_name: name,
    })?;

    if output.created {
        printer.success(&format!(
            "Created migration `{}` with {} operations (up + down).",
            output.migration_dir, output.operation_count
        ));
    } else {
        printer.info("No schema changes detected.");
    }

    Ok(())
}

pub fn run_create(context: &AppContext, name: &str, printer: &Printer) -> Result<()> {
    let output = context.migrate_create.execute(MigrateCreateInput {
        migrations_dir: context.migrations_dir.clone(),
        name: name.to_owned(),
    })?;

    printer.success(&format!("Created migration `{}`.", output.migration_dir));
    Ok(())
}

pub fn run_status(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.migrate_status.execute(MigrateStatusInput)?;

    if output.migrations.is_empty() {
        printer.info("No migrations found.");
    } else {
        printer.info("Migrations:");
        for migration in &output.migrations {
            printer.plain(&format!("  - {migration}"));
        }
    }

    if output.has_snapshot {
        printer.info("Latest snapshot is available.");
    }

    Ok(())
}

pub fn run_apply(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.migrate_apply.execute(MigrateApplyInput)?;
    printer.success(&format!("Applied {} migrations.", output.applied));
    Ok(())
}
