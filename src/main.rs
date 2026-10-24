use std::env;
use std::process::ExitCode;

use crate::editor::Editor;

mod editor;
mod file;
mod shortcuts;
mod syntax_cache;
mod terminal_editor;
mod ui_types;

struct Cli {
    file_path: String,
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Cli, String> {
    let mut args = args;
    let program = args.next().unwrap_or_else(|| "termcode".to_string());

    let Some(file_path) = args.next() else {
        return Err(format!("Usage: {program} <file>"));
    };

    if args.next().is_some() {
        return Err(format!("Usage: {program} <file>"));
    }

    Ok(Cli { file_path })
}

fn read_file(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Builds the editor for an existing file, with the cursor at the end of the document.
fn editor_for_file(contents: &str) -> Editor {
    let mut editor = Editor::from_text(contents);
    editor.move_cursor_to_end();
    editor
}

fn main() -> ExitCode {
    match parse_args(env::args()) {
        Ok(cli) => match read_file(&cli.file_path) {
            Ok(contents) => {
                let editor = editor_for_file(&contents);

                match terminal_editor::run(editor, cli.file_path.clone()) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("Terminal error: {error}");
                        ExitCode::FAILURE
                    }
                }
            }
            Err(_) => {
                let editor = Editor::from_text("");
                match terminal_editor::run(editor, cli.file_path.clone()) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("Terminal error: {error}");
                        ExitCode::FAILURE
                    }
                }
            }
        },
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file_path() -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "termcode-test-{}-{timestamp}.txt",
            std::process::id()
        ))
    }

    fn args<'a>(values: &'a [&str]) -> impl Iterator<Item = String> + 'a {
        values.iter().map(|value| (*value).to_string())
    }

    #[test]
    fn accepts_a_file_path() {
        let cli = parse_args(args(&["termcode", "example.txt"])).unwrap();

        assert_eq!(cli.file_path, "example.txt");
    }

    #[test]
    fn rejects_missing_file_path() {
        let result = parse_args(args(&["termcode"]));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_additional_arguments() {
        let result = parse_args(args(&["termcode", "one.txt", "two.txt"]));

        assert!(result.is_err());
    }

    #[test]
    fn error_no_such_file() {
        let path = temp_file_path();
        let result = read_file(path.to_str().unwrap());

        assert!(result.is_err());
    }

    #[test]
    fn reads_file_contents() {
        let path = temp_file_path();
        let expected = "Hello, termcode!\nHello from the 2nd line!";

        fs::write(&path, expected).unwrap();

        let result = read_file(path.to_str().unwrap());

        assert_eq!(result.unwrap(), expected);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn opened_file_starts_with_cursor_at_end() {
        let path = temp_file_path();
        fs::write(&path, "fn main() {\n    println!(\"hi\");\n}").unwrap();

        let editor = editor_for_file(&read_file(path.to_str().unwrap()).unwrap());

        assert_eq!(editor.cursor, editor::Cursor { row: 2, col: 1 });

        fs::remove_file(path).unwrap();
    }
}
