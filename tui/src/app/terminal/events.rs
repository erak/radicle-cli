use crate::app::terminal::keys::Key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
}
