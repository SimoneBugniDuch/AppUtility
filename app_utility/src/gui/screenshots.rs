use screenshots::Screen;

pub struct Screenshots {
    screenshots: Vec<Screen>,
    screen_number: usize,
}

impl Screenshots {
    pub fn new() -> Self {
        Self {
            screenshots: Screen::all().unwrap(),
            screen_number: 0,
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screenshots[self.screen_number]
    }
}