use anyhow::Result;
use schema_core::{UpgradeCliInput, UpgradeSource};

use crate::app::UpgradeFrom;
use crate::di::AppContext;
use crate::output::Printer;

pub fn run(
    context: &AppContext,
    check: bool,
    version: Option<String>,
    from: UpgradeFrom,
    printer: &Printer,
) -> Result<()> {
    let source = match from {
        UpgradeFrom::Github => UpgradeSource::Github,
        UpgradeFrom::CratesIo => UpgradeSource::CratesIo,
    };
    let target = crate::adapters::binary_installer::rust_target().unwrap_or("").to_owned();
    let output = context.upgrade_cli.execute(UpgradeCliInput {
        current_version: env!("CARGO_PKG_VERSION").to_owned(),
        check_only: check,
        version,
        source,
        target,
    })?;
    if output.updated {
        printer.success(&output.message);
    } else {
        printer.info(&output.message);
    }
    if !output.updated && output.latest_version != output.current_version && check {
        printer.info(&format!(
            "current={} latest={}",
            output.current_version, output.latest_version
        ));
    }
    Ok(())
}
