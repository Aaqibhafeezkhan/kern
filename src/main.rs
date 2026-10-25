use clap::Parser;
use std::process::ExitCode;

use crate::cli::Cli;
use crate::editor::editor_for_file;
use crate::file::read_file;

mod cli;
mod editor;
mod file;
mod shortcuts;
mod syntax_cache;
mod terminal_editor;
mod ui_types;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let contents = read_file(&cli.path).unwrap_or_default();

    let editor = editor_for_file(&contents);

    match terminal_editor::run(editor, cli.path) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Terminal error: {error}");
            ExitCode::FAILURE
        }
    }
}
