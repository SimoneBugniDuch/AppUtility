use screenshots::Screen;

pub struct Screenshots {
    screenshots: Vec<Screen>,
    pub screen_number: usize,
    pub default: bool,
    pub default_screen_number: usize,
}

impl Screenshots {
    pub fn new() -> Self {
        Self {
            screenshots: Screen::all().unwrap(),
            screen_number: 0,
            default: true,
            default_screen_number: 0,
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screenshots[self.screen_number]
    }

    pub fn total_screens(&self) -> usize {
        self.screenshots.len() as usize
    }
}