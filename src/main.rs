use std::process::ExitCode;

use clap::Parser;
use kern::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    cli.run()
}
