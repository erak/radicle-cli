pub struct Context<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> Context<'a> {
    pub fn new(title: &'a str) -> Context<'a> {
        Context {
            title,
            should_quit: false,
        }
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn on_tick(&mut self) {
        // Update state
    }
}