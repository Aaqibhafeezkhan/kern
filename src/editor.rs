#[derive(Debug, PartialEq, Eq)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Editor {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub last_input_char: char,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EditorError {
    InvalidCursorPosition { row: usize, col: usize },
}

impl Editor {
    pub fn from_text(text: &str) -> Self {
        let lines = text
            .split('\n')
            .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
            .collect();

        Self {
            lines,
            cursor: Cursor { row: 0, col: 0 },
            last_input_char: '_',
        }
    }

    /// Places the cursor after the last character of the last line, so an
    /// opened document can be appended to straight away.
    pub fn move_cursor_to_end(&mut self) {
        let row = self.lines.len().saturating_sub(1);
        let col = self.lines.get(row).map_or(0, |line| line.chars().count());
        self.cursor = Cursor { row, col };
    }

    fn validate_cursor(&self) -> Result<(), EditorError> {
        let Some(line) = self.lines.get(self.cursor.row) else {
            return Err(EditorError::InvalidCursorPosition {
                row: self.cursor.row,
                col: self.cursor.col,
            });
        };

        if self.cursor.col > line.chars().count() {
            return Err(EditorError::InvalidCursorPosition {
                row: self.cursor.row,
                col: self.cursor.col,
            });
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn insert_char(&mut self, ch: char) -> Result<(), EditorError> {
        self.validate_cursor()?;

        // Handle \r\n (Windows Enter)
        if self.last_input_char == '\r' && ch == '\n' {
            // Does nothing
            self.last_input_char = ch;
            return Ok(());
        }

        // Handle \r and \n (newline chars)
        if ch == '\n' || ch == '\r' {
            self.last_input_char = ch;

            let last_line = self.lines.remove(self.cursor.row);
            let first = last_line.chars().take(self.cursor.col).collect::<String>();
            let second = last_line.chars().skip(self.cursor.col).collect::<String>();

            self.lines.insert(self.cursor.row, first);
            self.lines.insert(self.cursor.row + 1, second);

            self.cursor.row += 1;
            self.cursor.col = 0;

            return Ok(());
        }

        let line = &mut self.lines[self.cursor.row];
        let byte_index = line
            .char_indices()
            .nth(self.cursor.col)
            .map_or(line.len(), |(index, _)| index);

        line.insert(byte_index, ch);
        self.cursor.col += 1;
        self.last_input_char = ch;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_char(&mut self) -> Result<(), EditorError> {
        self.validate_cursor()?;

        // When trying to delete at start of a line
        if self.cursor.row == 0 && self.cursor.col == 0 {
            // Do nothing
            return Ok(());
        } else if self.cursor.col == 0 {
            let second = self.lines.remove(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
            self.lines[self.cursor.row].push_str(&second);

            return Ok(());
        }

        let line = &mut self.lines[self.cursor.row];

        let Some(start) = line
            .char_indices()
            .nth(self.cursor.col - 1)
            .map(|(index, _)| index)
        else {
            // Cursor ist at the end of the line
            return Ok(());
        };

        let end = line[start..]
            .char_indices()
            .nth(1)
            .map_or(line.len(), |(offset, _)| start + offset);

        line.replace_range(start..end, "");
        self.cursor.col -= 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
