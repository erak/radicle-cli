use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;

use crossterm::event;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Enter,
    Esc,
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
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,
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

pub enum InputEvent {
    Input(Key),
    Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: Receiver<InputEvent>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    let key = Key::from(key);
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
