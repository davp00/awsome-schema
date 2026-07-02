use anyhow::Result;

use schema_core::FormatSchemaInput;

use crate::di::AppContext;
use crate::output::Printer;

pub fn run(context: &AppContext, write: bool, printer: &Printer) -> Result<()> {
    let output = context.format_schema.execute(FormatSchemaInput { write_back: write })?;

    if write {
        printer.success(&format!("Formatted `{}`.", context.schema_path));
    } else {
        printer.plain(&output.formatted);
    }

    Ok(())
}
