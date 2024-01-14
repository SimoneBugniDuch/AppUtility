use eframe::egui::style::Spacing;
use eframe::egui::{self, Color32, Style, TopBottomPanel, Visuals, CentralPanel};
use eframe::{run_native, App, Frame, NativeOptions};

#[derive(Default)]
struct AppUtility {}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::light());
        Default::default()
    }

    fn configure_ui_style(&self, ctx: &egui::Context) {
        let mut style: Style = (*ctx.style()).clone();
        // Customize the button visuals
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(200); // Background color
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(10.0); // Rounded corners
        style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::BLACK; // Text color

        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(100, 100, 100); // Background color
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(10.0); // Rounded corners
        style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::BLACK; // Text color

        // Add padding around the text
        style.spacing.button_padding = egui::Vec2::new(10.0, 5.0); // Horizontal and Vertical padding

        // Customize the button font size
        if let Some(button_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
            button_style.size = 20.0;
        }

        ctx.set_style(style);
    }
}

impl App for AppUtility {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.configure_ui_style(ctx);
        
        TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.add_space(3.0);
            if ui.button("+ NEW")
            .on_hover_text("Take a new screenshot")
            .clicked() {
                println!("new");
            }
            ui.add_space(3.0);
        });
        
        CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Hello World!"); 
        });

    }
}

pub fn window() -> eframe::Result<()> {
    // Set the main window configuration options
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("AppUtility")
            .with_inner_size(eframe::egui::Vec2::new(800.0, 300.0)) // Set the initial window size
            .with_app_id("app_utility".to_owned()), // Set the application ID,
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        run_and_return: false, // Determines app behavior when main window is closed
        centered: true,        // Center the window on startup
        ..Default::default()
    };

    run_native(
        "AppUtility",
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| Box::new(AppUtility::new(cc))),
    )
}
