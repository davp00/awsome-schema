use anyhow::Result;

use schema_core::{GenerateCodeInput, GenerateCodeTarget};

use crate::app::GenerateTarget as CliGenerateTarget;
use crate::di::AppContext;
use crate::output::Printer;

pub fn run(
    context: &AppContext,
    target: CliGenerateTarget,
    stdout: bool,
    printer: &Printer,
) -> Result<()> {
    let target = match target {
        CliGenerateTarget::Schema => GenerateCodeTarget::Schema,
        CliGenerateTarget::Rust => GenerateCodeTarget::Rust,
        CliGenerateTarget::Typescript => GenerateCodeTarget::TypeScript,
    };

    let output = context.generate_code.execute(GenerateCodeInput { target, stdout })?;

    if let Some(path) = &output.written_path {
        printer.success(&format!("Wrote TypeScript client to `{path}`."));
    } else {
        printer.plain(&output.content);
    }

    Ok(())
}
