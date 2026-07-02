use anyhow::Result;

use schema_core::{DbPullInput, DbPushInput};

use crate::di::AppContext;
use crate::output::Printer;

pub fn run_pull(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.db_pull.execute(DbPullInput)?;
    printer.success(&format!("Pulled schema with {} models.", output.schema.models.len()));
    Ok(())
}

pub fn run_push(context: &AppContext, _printer: &Printer) -> Result<()> {
    context.db_push.execute(DbPushInput)?;
    Ok(())
}
