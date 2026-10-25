use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::Cli;
use crate::editor::Editor;

mod cli;
mod editor;
mod file;
mod shortcuts;
mod syntax_cache;
mod terminal_editor;
mod ui_types;

fn read_file(path: &PathBuf) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Builds the editor for an existing file, with the cursor at the end of the document.
fn editor_for_file(contents: &str) -> Editor {
    let mut editor = Editor::from_text(contents);
    editor.move_cursor_to_end();
    editor
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file_path() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("kern-test-{}-{timestamp}.txt", std::process::id()))
    }

    #[test]
    fn accepts_a_file_path() {
        let cli = Cli::try_parse_from(["kern", "example.txt"]).unwrap();

        assert_eq!(cli.path, PathBuf::from("example.txt"));
    }

    #[test]
    fn rejects_missing_file_path() {
        let result = Cli::try_parse_from(["kern"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_additional_arguments() {
        let result = Cli::try_parse_from(["kern", "one.txt", "two.txt"]);

        assert!(result.is_err());
    }

    #[test]
    fn accepts_short_help_flag() {
        let result = Cli::try_parse_from(["kern", "-h"]);

        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn accepts_long_help_flag() {
        let result = Cli::try_parse_from(["kern", "--help"]);

        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn error_no_such_file() {
        let path = temp_file_path();
        let result = read_file(&path);

        assert!(result.is_err());
    }

    #[test]
    fn reads_file_contents() {
        let path = temp_file_path();
        let expected = "Hello, kern!\nHello from the 2nd line!";

        fs::write(&path, expected).unwrap();

        let result = read_file(&path);

        assert_eq!(result.unwrap(), expected);

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn opened_file_starts_with_cursor_at_end() {
        let path = temp_file_path();
        fs::write(&path, "fn main() {\n    println!(\"hi\");\n}").unwrap();

        let contents = read_file(&path).unwrap();
        let editor = editor_for_file(&contents);

        assert_eq!(editor.cursor, editor::Cursor { row: 2, col: 1 });

        fs::remove_file(&path).unwrap();
    }
}
