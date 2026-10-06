#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::if_same_then_else
)]

pub mod adapters;
pub mod app;
pub mod commands;
pub mod di;
pub mod output;
pub mod schema_layout;

use clap::Parser;

use app::Cli;
use output::Printer;

pub use di::test_context_with_introspector;

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let printer = Printer::new();
    commands::dispatch(&cli, &printer)
}
