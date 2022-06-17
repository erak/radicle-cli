use std::cmp;
// use unicode_segmentation::UnicodeSegmentation;

use tui::layout::Rect;

use super::events::Key;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.len()
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    // pub fn update_len(&mut self) {
    //     self.len = self.string[..].graphemes(true).count();
    // } 

    // pub fn insert(&mut self, at: usize, c: char) {
    //     if at >= self.len() {
    //         self.string.push(c);
    //     } else {
    //         let mut result: String = self.string[..].graphemes(true).take(at).collect();
    //         let remainder: String = self.string[..].graphemes(true).skip(at).collect();
    //         result.push(c);
    //         result.push_str(&remainder);
    //         self.string = result;
    //     }
    //     self.update_len();
    // }

    // pub fn len(&self) -> usize {
    //     self.len
    // }
}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn new(content: String) -> Self {
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Self { rows }
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

    // pub fn insert(&mut self, at: &Position, c: char) {
    //     if at.y == self.len() {
    //         let mut row = Row::default();
    //         row.insert(0, c);
    //         self.rows.push(row);
    //     } else if at.y < self.len() {
    //         let row = self.rows.get_mut(at.y).unwrap();
    //         row.insert(at.x, c);
    //     }
    // }
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    cursor_position: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            cursor_position: Position::default(),
            document: Document::new(String::new()),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.cursor_position = Position::default();
        self.document = Document::new(content);
    }

    // pub fn clear(&mut self) {
    //     self.cursor_position = Position::default();
    //     self.document = Document::new(String::new())
    // }

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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // Terminal::cursor_hide();
        // Terminal::cursor_position(&Position::default());

        // if self.should_quit {
        //     Terminal::clear_screen();
        //     println!("Goodbye.\r");
        // } else {
        //     self.draw_rows();
        //     Terminal::cursor_position(&self.cursor_position);
        // }

        // Terminal::cursor_show();
        // Terminal::flush()
        Ok(())
    }

    pub fn move_cursor(&mut self, area: Rect, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = area.height.saturating_sub(1) as usize;
        let width = area.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            // Key::PageUp => y = 0,
            // Key::PageDown => y = height,
            // Key::Home => x = 0,
            // Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }
}
