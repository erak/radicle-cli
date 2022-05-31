mod widgets;

use crate::app::App;
use widgets::{Widget, MenuWidget};

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3)].as_ref())
        .split(frame.size());

    let menu = MenuWidget::new(app);
    menu.draw(frame, chunks[0]);
}
