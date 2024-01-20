use std::time::Instant;

pub struct Timer {
    pub seconds: usize,
    form_open: bool,
    running: bool,
    pub start_instant: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            seconds: 0,
            form_open: false,
            running: false,
            start_instant: None,
        }
    }

    pub fn form_opened(&self) -> bool {
        self.form_open
    }

    pub fn open_form(&mut self) {
        self.form_open = true;
    }

    pub fn close_form(&mut self) {
        self.form_open = false;
    }

    pub fn start_timer(&mut self) {
        self.form_open = false;
        self.running = true;
        self.start_instant = Some(Instant::now());
    }

    pub fn decrement_timer(&mut self) {
        self.seconds -= 1;
    }

    pub fn reset_timer(&mut self) {
        self.seconds = 0;
        self.form_open = false;
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}