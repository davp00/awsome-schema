use anyhow::Result;

use schema_core::DbPullInput;

use crate::di::AppContext;
use crate::output::Printer;

pub fn run_pull(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.db_pull.execute(DbPullInput)?;
    printer.success(&format!("Pulled schema with {} models.", output.schema.models.len()));
    Ok(())
}

pub fn run_push(context: &AppContext, printer: &Printer) -> Result<()> {
    let schema = context.load_schema()?;
    let output =
        context.db_push.execute(schema_core::DbPushInput { datasource: schema.datasource })?;
    printer.success(&format!("Pushed schema to database ({} bytes).", output.statements.len()));
    Ok(())
}
