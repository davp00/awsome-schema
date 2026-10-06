use anyhow::Result;

use schema_core::{
    MigrateCreateInput, MigrateDevInput, MigrateRollbackInput, MigrateStatusInput,
    MigrationApplyState,
};

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
    let schema = context.load_schema()?;
    let output = context.migrate_status.execute(MigrateStatusInput {
        datasource: schema.datasource,
    })?;

    if output.migrations.is_empty() {
        printer.info("No migrations found.");
    } else {
        printer.info("Migrations:");
        for row in &output.migrations {
            let state = match row.state {
                MigrationApplyState::Applied => "applied",
                MigrationApplyState::Pending => "pending",
            };
            match &row.applied_at {
                Some(applied_at) => {
                    printer.plain(&format!("  - {} [{state}] ({applied_at})", row.name));
                }
                None => printer.plain(&format!("  - {} [{state}]", row.name)),
            }
        }
        printer.info(&format!(
            "Summary: {} applied, {} pending.",
            output.applied_count, output.pending_count
        ));
    }

    if output.has_snapshot {
        printer.info("Latest snapshot is available.");
    }

    Ok(())
}

pub fn run_apply(context: &AppContext, printer: &Printer) -> Result<()> {
    let schema = context.load_schema()?;
    let output = context.migrate_apply.execute(schema_core::MigrateApplyInput {
        migrations_dir: context.migrations_dir.clone(),
        datasource: schema.datasource,
    })?;
    printer.success(&format!(
        "Applied {} migrations ({} already applied).",
        output.applied, output.skipped
    ));
    Ok(())
}

pub fn run_rollback(context: &AppContext, steps: usize, printer: &Printer) -> Result<()> {
    let schema = context.load_schema()?;
    let output = context.migrate_rollback.execute(MigrateRollbackInput {
        migrations_dir: context.migrations_dir.clone(),
        datasource: schema.datasource,
        steps,
    })?;

    if output.rolled_back.is_empty() {
        printer.info("No migrations rolled back.");
    } else {
        printer.success(&format!(
            "Rolled back {} migration(s): {}.",
            output.rolled_back.len(),
            output.rolled_back.join(", ")
        ));
    }

    Ok(())
}
