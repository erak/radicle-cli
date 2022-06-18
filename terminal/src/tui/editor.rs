use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

use tui::layout::Rect;

use super::events::Key;

#[derive(Clone, Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;
        Self {
            string: splitted_row,
            len: splitted_length,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

#[derive(Clone, Default)]
pub struct Document {
    rows: Vec<Row>,
    dirty: bool,
}

impl Document {
    pub fn new(content: String) -> Self {
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Self {
            rows: rows,
            dirty: false,
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[at.y].split(at.x);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }

    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
}

#[derive(Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Editor {
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            cursor_position: Position::default(),
            document: Document::new(String::new()),
            offset: Position::default(),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.cursor_position = Position::default();
        self.document = Document::new(content);
    }

    pub fn render_row(&self, row: &Row, area: Rect) -> String {
        let start = 0;
        let end = area.width as usize;
        row.render(start, end)
    }

    pub fn render_rows(&self, area: Rect) -> Vec<String> {
        let height = area.height;
        let mut rendered_rows = vec![];
        for terminal_row in 0..height - 1 {
            if let Some(row) = self.document.row(terminal_row as usize) {
                rendered_rows.push(self.render_row(row, area));
            }
        }
        rendered_rows
    }

    pub fn process_keypress(&mut self, key: Key, area: Rect) -> Result<(), std::io::Error> {
        match key {
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right, area);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left, area);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(key, area),
            Key::ShiftEnter => self.move_cursor(Key::Down, area),
            // | Key::PageUp
            // | Key::PageDown
            // | Key::End
            // | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll(area);
        Ok(())
    }

    fn scroll(&mut self, area: Rect) {
        let Position { x, y } = self.cursor_position;
        let width = area.width as usize;
        let height = area.height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key, area: Rect) {
        let terminal_height = area.height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            // Key::PageUp => {
            //     y = if y > terminal_height {
            //         y.saturating_sub(terminal_height)
            //     } else {
            //         0
            //     }
            // }
            // Key::PageDown => {
            //     y = if y.saturating_add(terminal_height) < height {
            //         y.saturating_add(terminal_height)
            //     } else {
            //         height
            //     }
            // }
            // Key::Home => x = 0,
            // Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    pub fn cursor_position(&self) -> &Position {
        &self.cursor_position
    }
}
