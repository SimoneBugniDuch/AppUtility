use eframe::{NativeOptions, run_native, App, Frame};
use eframe::egui::{Context, TopBottomPanel, CentralPanel, Layout, Align, Button, ScrollArea};
use screenshots::display_info;



#[derive(PartialEq, Eq)]
enum Action {
    None,   
    Screenshot,
}
struct AppUtility {
    display: Option<usize>,
    action: Action,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _context = cc.egui_ctx.clone();

        AppUtility {
            display: Some(0),
            action: Action::None,
        }
    }
    fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if self.action == Action::None {
                    let screenshot_btn: eframe::egui::Response = ui.add(Button::new("➕ New")).on_hover_text("Take screenshot of selected display");
                    if screenshot_btn.clicked() {
                        self.action = Action::Screenshot;
                    }
                    
            } else {
                let cancel_btn = ui.add(Button::new("❌ Cancel")).on_hover_text("Cancel");
                if cancel_btn.clicked() {
                    self.action = Action::None;
                }
            }
            });
        });
    }

    fn render_central_panel(&mut self, ctx: &Context, _frame: &mut Frame) {
    }

}

impl App for AppUtility {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_top_panel(ctx, frame);
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