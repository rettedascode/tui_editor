use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ropey::Rope;
use std::cmp;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub cursor: Position,
    pub scroll_offset: Position,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            cursor: Position::new(0, 0),
            scroll_offset: Position::new(0, 0),
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent, content: &mut Rope) {
        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.handle_ctrl_char(c, content);
                } else {
                    self.insert_char(c, content);
                }
            }
            KeyCode::Backspace => {
                self.delete_char(content);
            }
            KeyCode::Delete => {
                self.delete_char_forward(content);
            }
            KeyCode::Enter => {
                self.insert_newline(content);
            }
            KeyCode::Up => {
                self.move_cursor_up(content);
            }
            KeyCode::Down => {
                self.move_cursor_down(content);
            }
            KeyCode::Left => {
                self.move_cursor_left(content);
            }
            KeyCode::Right => {
                self.move_cursor_right(content);
            }
            KeyCode::Home => {
                self.move_to_line_start();
            }
            KeyCode::End => {
                self.move_to_line_end(content);
            }
            KeyCode::PageUp => {
                self.page_up(content);
            }
            KeyCode::PageDown => {
                self.page_down(content);
            }
            _ => {}
        }
    }

    fn handle_ctrl_char(&mut self, c: char, content: &mut Rope) {
        match c {
            'a' => self.select_all(content),
            'c' => self.copy_selection(content),
            'v' => self.paste(content),
            'x' => self.cut_selection(content),
            'z' => self.undo(),
            'y' => self.redo(),
            _ => {}
        }
    }

    fn insert_char(&mut self, c: char, content: &mut Rope) {
        let char_idx = self.get_char_index(content);
        content.insert_char(char_idx, c);
        self.cursor.col += 1;
    }

    fn insert_newline(&mut self, content: &mut Rope) {
        let char_idx = self.get_char_index(content);
        content.insert_char(char_idx, '\n');
        self.cursor.row += 1;
        self.cursor.col = 0;
    }

    fn delete_char(&mut self, content: &mut Rope) {
        if self.cursor.col > 0 {
            let char_idx = self.get_char_index(content);
            content.remove(char_idx - 1..char_idx);
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            // Join with previous line
            let line_start = content.line_to_char(self.cursor.row);
            let prev_line_len = content.line(self.cursor.row - 1).len_chars();

            content.remove(line_start - 1..line_start);
            self.cursor.row -= 1;
            self.cursor.col = prev_line_len;
        }
    }

    fn delete_char_forward(&mut self, content: &mut Rope) {
        let char_idx = self.get_char_index(content);
        if char_idx < content.len_chars() {
            content.remove(char_idx..char_idx + 1);
        }
    }

    fn move_cursor_up(&mut self, content: &Rope) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            let line_len = content.line(self.cursor.row).len_chars();
            self.cursor.col = cmp::min(self.cursor.col, line_len);
        }
    }

    fn move_cursor_down(&mut self, content: &Rope) {
        if self.cursor.row < content.len_lines().saturating_sub(1) {
            self.cursor.row += 1;
            let line_len = content.line(self.cursor.row).len_chars();
            self.cursor.col = cmp::min(self.cursor.col, line_len);
        }
    }

    fn move_cursor_left(&mut self, content: &Rope) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = content.line(self.cursor.row).len_chars();
        }
    }

    fn move_cursor_right(&mut self, content: &Rope) {
        let line_len = content.line(self.cursor.row).len_chars();
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.row < content.len_lines().saturating_sub(1) {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    fn move_to_line_start(&mut self) {
        self.cursor.col = 0;
    }

    fn move_to_line_end(&mut self, content: &Rope) {
        self.cursor.col = content.line(self.cursor.row).len_chars();
    }

    fn page_up(&mut self, content: &Rope) {
        // Move up by 10 lines or to top
        for _ in 0..10 {
            self.move_cursor_up(content);
        }
    }

    fn page_down(&mut self, content: &Rope) {
        // Move down by 10 lines or to bottom
        for _ in 0..10 {
            self.move_cursor_down(content);
        }
    }

    fn get_char_index(&self, content: &Rope) -> usize {
        content.line_to_char(self.cursor.row) + self.cursor.col
    }

    // Placeholder methods for advanced features
    fn select_all(&mut self, _content: &Rope) {
        // TODO: Implement selection
    }

    fn copy_selection(&mut self, _content: &Rope) {
        // TODO: Implement copy
    }

    fn paste(&mut self, _content: &mut Rope) {
        // TODO: Implement paste
    }

    fn cut_selection(&mut self, _content: &mut Rope) {
        // TODO: Implement cut
    }

    fn undo(&mut self) {
        // TODO: Implement undo
    }

    fn redo(&mut self) {
        // TODO: Implement redo
    }

    pub fn get_visible_lines(&self, content: &Rope, height: usize) -> Vec<String> {
        let start_line = self.scroll_offset.row;
        let end_line = cmp::min(start_line + height, content.len_lines());

        (start_line..end_line)
            .map(|i| content.line(i).to_string())
            .collect()
    }

    pub fn ensure_cursor_visible(&mut self, _content: &Rope, width: usize, height: usize) {
        // Ensure cursor is within visible area
        if self.cursor.row < self.scroll_offset.row {
            self.scroll_offset.row = self.cursor.row;
        } else if self.cursor.row >= self.scroll_offset.row + height {
            self.scroll_offset.row = self.cursor.row.saturating_sub(height - 1);
        }

        if self.cursor.col < self.scroll_offset.col {
            self.scroll_offset.col = self.cursor.col;
        } else if self.cursor.col >= self.scroll_offset.col + width {
            self.scroll_offset.col = self.cursor.col.saturating_sub(width - 1);
        }
    }
}
