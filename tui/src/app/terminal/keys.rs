use crossterm::event;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
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
            _ => Key::Unknown,
        }
    }
}