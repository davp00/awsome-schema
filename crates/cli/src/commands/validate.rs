use anyhow::Result;

use schema_core::ValidateSchemaInput;

use crate::di::AppContext;
use crate::output::Printer;

pub fn run(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.validate_schema.execute(ValidateSchemaInput)?;

    printer.success(&format!(
        "Schema is valid ({} models, {} edges).",
        output.model_count, output.edge_count
    ));

    Ok(())
}
