use crossterm::event;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Up,
    Down,
    Char(char),
    Unknown,
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            _ => Key::Unknown,
        }
    }
}