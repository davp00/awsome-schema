use anyhow::Result;

use schema_core::FormatSchemaInput;

use crate::adapters::list_schema_files;
use crate::di::AppContext;
use crate::output::Printer;

pub fn run(context: &AppContext, write: bool, printer: &Printer) -> Result<()> {
    let schema_files = list_schema_files(&*context.filesystem, &context.schema_path)?;
    let output = context.format_schema.execute(FormatSchemaInput { write_back: write, schema_files })?;

    if write {
        if context.filesystem.is_directory(&context.schema_path) {
            printer.success(&format!("Formatted schema directory `{}`.", context.schema_path));
        } else {
            printer.success(&format!("Formatted `{}`.", context.schema_path));
        }
    } else {
        printer.plain(&output.formatted);
    }

    Ok(())
}
