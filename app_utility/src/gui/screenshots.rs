use eframe::WindowInfo;
use screenshots::Screen;

pub struct Screenshots {
    screenshots: Vec<Screen>,
    pub screen_number: usize,
    pub default: bool,
    default_screen_number: usize,
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

    pub fn set_screen_number(&mut self, info: WindowInfo) {
        let x = info.position.unwrap().x as i32;
        let y = info.position.unwrap().y as i32 - (info.monitor_size.unwrap().y as i32 - info.size.y as i32);
        match Screen::from_point(x, y) {
            Ok(screen_info) => {
                let id = screen_info.display_info.id;
                let all_screens = Screen::all().unwrap();
                if let Some(i) = all_screens.iter().position(|&screen| screen.display_info.id == id) {
                    self.screen_number = i;
                    self.default = false;
                }
            },
            Err(_) => {},
        }
    }

    pub fn get_screen(&self) -> Screen {
        self.screenshots[self.screen_number]
    }

    pub fn get_default_screen(&self) -> Screen {
        self.screenshots[self.default_screen_number]
    }

    pub fn total_screens(&self) -> usize {
        self.screenshots.len() as usize
    }
}