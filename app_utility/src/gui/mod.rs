use std::sync::mpsc::{Sender, Receiver, channel};

use eframe::{NativeOptions, run_native, App, Frame};
use eframe::egui::{Align, Context, TopBottomPanel, Layout};
use image::DynamicImage;
use screenshots::display_info;

#[derive(PartialEq, Eq)]
enum Action {
    None,
    Screenshot
}

struct AppUtility {
    display: Option<usize>,
    timer: Option<usize>,
    sender: Sender<DynamicImage>,
    receiver: Receiver<DynamicImage>,
    current_screen: Option<DynamicImage>,
    action: Action,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let context = cc.egui_ctx.clone();
        let (sender, receiver) = channel();

        AppUtility {
            display: Some(0),
            timer: Some(0),
            sender,
            receiver,
            current_screen: None,
            action: Action::None
        }
    }

    fn top_panel_view(&mut self, ctx: &Context, frame: &mut Frame) {
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
}

impl App for AppUtility {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // match self.receiver.try_recv() {
        //     Ok(screenshot) => {
        //         self.current_screen = Some(screenshot);
        //     },
        //     Err(_) => {}
        // };
        self.top_panel_view(ctx, frame);
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