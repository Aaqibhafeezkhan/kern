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

/// Builds the editor for an existing file, with the cursor at the end of the document.
pub fn editor_for_file(contents: &str) -> Editor {
    let mut editor = Editor::from_text(contents);
    editor.move_cursor_to_end();
    editor
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
