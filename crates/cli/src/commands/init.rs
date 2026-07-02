use anyhow::Result;

use schema_core::InitProjectInput;

use crate::di::AppContext;
use crate::output::Printer;

pub fn run(context: &AppContext, printer: &Printer) -> Result<()> {
    let output = context.init_project.execute(InitProjectInput {
        schema_path: context.schema_path.clone(),
        migrations_dir: context.migrations_dir.clone(),
    })?;

    if output.schema_created {
        printer.success(&format!(
            "Created `{}` and `{}` directory.",
            context.schema_path, context.migrations_dir
        ));
    } else {
        printer.info(&format!(
            "`{}` already exists. Ensured `{}` directory exists.",
            context.schema_path, context.migrations_dir
        ));
    }

    Ok(())
}
