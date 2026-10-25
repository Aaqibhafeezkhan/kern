mod common;

use common::temp_file_path;
use kern::editor;
use kern::editor::{Cursor, Editor, EditorError};
use kern::file::read_file;
use std::fs;

#[test]
fn opened_file_starts_with_cursor_at_end() {
    let path = temp_file_path();
    fs::write(&path, "fn main() {\n    println!(\"hi\");\n}").unwrap();

    let contents = read_file(&path).unwrap();
    let editor = editor::editor_for_file(&contents);

    assert_eq!(editor.cursor, editor::Cursor { row: 2, col: 1 });

    fs::remove_file(&path).unwrap();
}

#[test]
fn creates_editor_from_multipe_lines() {
    let editor = Editor::from_text("first\nSecond");

    assert_eq!(
        editor.lines,
        vec!["first".to_string(), "Second".to_string()]
    );

    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
}

#[test]
fn empty_text_contains_one_empty_line() {
    let editor = Editor::from_text("");

    assert_eq!(editor.lines, vec!["".to_string()]);
    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
}

#[test]
fn preserves_trailing_newline_as_empty_line() {
    let editor = Editor::from_text("first\n");

    assert_eq!(editor.lines, vec!["first".to_string(), "".to_string()]);
    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
}

#[test]
fn handled_windows_line_endings() {
    let editor = Editor::from_text("first\r\nsecond\r\n");

    assert_eq!(
        editor.lines,
        vec!["first".to_string(), "second".to_string(), "".to_string()]
    );

    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
}

#[test]
fn moves_cursor_to_end_of_multiple_lines() {
    let mut editor = Editor::from_text("first\nsecond line");

    editor.move_cursor_to_end();

    assert_eq!(editor.cursor, Cursor { row: 1, col: 11 });
    assert_eq!(editor.insert_char('!'), Ok(()));
    assert_eq!(editor.lines[1], "second line!");
}

#[test]
fn moves_cursor_to_end_after_trailing_newline() {
    let mut editor = Editor::from_text("first\r\nsecond\r\n");

    editor.move_cursor_to_end();

    assert_eq!(editor.cursor, Cursor { row: 2, col: 0 });
}

#[test]
fn moves_cursor_to_end_counting_characters_not_bytes() {
    let mut editor = Editor::from_text("a🦀é");

    editor.move_cursor_to_end();

    assert_eq!(editor.cursor, Cursor { row: 0, col: 3 });
    assert_eq!(editor.delete_char(), Ok(()));
    assert_eq!(editor.lines[0], "a🦀");
}

#[test]
fn moves_cursor_to_end_of_empty_text() {
    let mut editor = Editor::from_text("");

    editor.move_cursor_to_end();

    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
    assert_eq!(editor.insert_char('x'), Ok(()));
    assert_eq!(editor.lines, vec!["x".to_string()]);
}

#[test]
fn inserts_character_at_cursor() {
    let mut editor = Editor::from_text("ac");
    editor.cursor.col = 1;

    editor.insert_char('b').unwrap();

    assert_eq!(editor.lines[0], "abc");
    assert_eq!(editor.cursor.col, 2);
}

#[test]
fn inserts_newline_at_cursor() {
    let mut editor = Editor::from_text("first");
    editor.cursor.col = 5;

    editor.insert_char('\n').unwrap();

    assert_eq!(editor.lines, vec!["first".to_string(), "".to_string()]);

    assert_eq!(editor.cursor, Cursor { row: 1, col: 0 });

    editor.cursor = Cursor { row: 0, col: 3 };

    editor.insert_char('\r').unwrap();
    editor.insert_char('\n').unwrap();

    assert_eq!(
        editor.lines,
        vec!["fir".to_string(), "st".to_string(), "".to_string()]
    );

    assert_eq!(editor.cursor, Cursor { row: 1, col: 0 });
}

#[test]
fn deletes_character_at_cursor() {
    let mut editor = Editor::from_text("abc");
    editor.cursor.col = 2;

    editor.delete_char().unwrap();

    assert_eq!(editor.lines[0], "ac");
    assert_eq!(editor.cursor.col, 1);
}

#[test]
fn deletes_at_line_start() {
    let mut editor = Editor::from_text("fir\nst\n");
    editor.cursor = Cursor { row: 2, col: 0 };

    editor.delete_char().unwrap();

    assert_eq!(editor.lines, vec!["fir".to_string(), "st".to_string()]);

    assert_eq!(editor.cursor, Cursor { row: 1, col: 2 });

    editor.cursor.col = 0;
    editor.delete_char().unwrap();

    assert_eq!(editor.lines, vec!["first".to_string()]);

    assert_eq!(editor.cursor, Cursor { row: 0, col: 3 });
}

#[test]
fn deletion_at_filestart_is_noop() {
    let mut editor = Editor::from_text("first");
    editor.cursor = Cursor { row: 0, col: 0 };

    editor.delete_char().unwrap();

    assert_eq!(editor.lines, vec!["first".to_string()]);

    assert_eq!(editor.cursor, Cursor { row: 0, col: 0 });
}

#[test]
fn inserts_unicode_character() {
    let mut editor = Editor::from_text("a🦀b");
    editor.cursor.col = 1;

    editor.insert_char('é').unwrap();

    assert_eq!(editor.lines[0], "aé🦀b");
    assert_eq!(editor.cursor.col, 2);
}

#[test]
fn deletes_unicode_character() {
    let mut editor = Editor::from_text("a🦀b");
    editor.cursor.col = 2;

    editor.delete_char().unwrap();

    assert_eq!(editor.lines[0], "ab");
}

#[test]
fn rejects_cursor_row_out_of_bounds() {
    let mut editor = Editor::from_text("abc");
    editor.cursor.row = 1;

    assert_eq!(
        editor.insert_char('x'),
        Err(EditorError::InvalidCursorPosition { row: 1, col: 0 })
    );
    assert_eq!(editor.lines[0], "abc");
}

#[test]
fn rejects_cursor_column_out_of_bounds() {
    let mut editor = Editor::from_text("abc");
    editor.cursor.col = 4;

    assert_eq!(
        editor.delete_char(),
        Err(EditorError::InvalidCursorPosition { row: 0, col: 4 })
    );
    assert_eq!(editor.lines[0], "abc");
}
