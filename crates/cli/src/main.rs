#![allow(
    clippy::doc_markdown,
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::if_same_then_else
)]

mod adapters;
mod app;
mod commands;
mod di;
mod output;

use clap::Parser;

use app::Cli;
use output::Printer;

fn main() {
    let cli = Cli::parse();
    let printer = Printer::new();

    let result = commands::dispatch(&cli, &printer);

    if let Err(error) = result {
        let message = error.to_string();
        printer.error(&message);
        std::process::exit(1);
    }
}
