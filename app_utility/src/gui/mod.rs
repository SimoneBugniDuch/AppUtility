use eframe::{NativeOptions, run_native, App, Frame};
use eframe::egui::{Context};
use screenshots::display_info;

struct AppUtility {
    display: Option<usize>,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _context = cc.egui_ctx.clone();

        AppUtility {
            display: Some(0),
        }
    }
}

impl App for AppUtility {
    fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {
        
    }
}

pub fn window() -> eframe::Result<()> {
    let options: NativeOptions = NativeOptions::default();
    run_native(
        "AppUtility", 
    options, 
    Box::new(|cc: &eframe::CreationContext<'_>| Box::new(AppUtility::new(cc)))
    )
}