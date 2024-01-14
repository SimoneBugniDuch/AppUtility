mod shortcut;
mod actions;
mod screenshots;

use std::time::Duration;

use chrono::Local;
// use eframe::egui::{self, CentralPanel, Color32, Style, TopBottomPanel, Visuals};
use eframe::{egui::{self, CentralPanel, Color32, Style, TopBottomPanel, Visuals, Window, Layout}, Frame, run_native, App, NativeOptions};
// use eframe::{run_native, App, Frame, NativeOptions};
use image;

use self::actions::Action;
use self::screenshots::Screenshots;
use self::shortcut::NewShortcut;

struct AppUtility {
    rectangle: Rectangle,
    default_path: String,
    new_shortcut: NewShortcut,
    default_name: String,
    hide: bool,
    screenshots: Screenshots,
    buffer: Option<Vec<u8>>,
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::light());
        // Default::default()
        Self {
            rectangle: Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            default_path: "screenshot".to_string(),
            new_shortcut: NewShortcut::default(),
            default_name: build_default_name(),
            hide: false,
            screenshots: Screenshots::new(),
            buffer: None,
        }
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
            button_style.size = 24.0;
        }

        ctx.set_style(style);
    }

    fn make_action(&mut self, action: Action, ctx: &egui::Context, frame: &mut Frame) {
        match action {
            Action::Capture => {
                self.hide = true;
                frame.set_visible(false);
            },
            Action::Close => {},
            Action::Copy => {},
            Action::Modify => {},
            Action::NewScreenshot => {},
            Action::Save => {},
            Action::Undo => {}
        }
    }
}

impl App for AppUtility {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.configure_ui_style(ctx);

        TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.add_space(6.0);

            // Use a horizontal layout to control the positioning of the menu button
            ui.horizontal(|ui| {
                ui.add_space(10.0); // Space on the left side of the menu button

                ui.menu_button("➕ NEW", |ui| {
                    ui.set_min_width(250.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.add_space(10.0);

                            // BUTTONS
                            if ui.button("🖵 Fullscreen shot").clicked() {
                                // TODO:
                                println!("full screenshot");
                            }
                            ui.add_space(6.0); // Space between buttons
                            if ui.button("⛶ Area shot").clicked() {
                                // TODO:
                                println!("area screenshot");
                            }

                            ui.add_space(10.0); // Space at the bottom of the menu
                        });
                    });
                });
                // This spacer will push the next items to the right
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(ui.available_width() - ui.spacing().item_spacing.x - 180.0);
                });

                if ui.button(" 🔧 SETTINGS").clicked() {}
                ui.add_space(10.0); // Space on the right side of the navbar
            });

            ui.add_space(6.0);
        });

        CentralPanel::default().show(ctx, |_ui| {
            // ui.heading("Hello World!");
        });

        if self.hide {
            std::thread::sleep(Duration::from_millis(400));
            let mut screen = self.screenshots.get_screen();
            let img = screen.capture().unwrap();
            self.buffer = Some(img.to_png(None).unwrap());
            frame.set_visible(false);
            Window::new("New screenshot")
                .title_bar(false)
                .show(ctx, |ui| {
                    ui.with_layout(
                        Layout {
                            main_dir: egui::Direction::LeftToRight,
                            main_wrap: false,
                            main_align: egui::Align::Center,
                            main_justify: false,
                            cross_align: egui::Align::Center,
                            cross_justify: false,
                        }, |ui| {

                        })
                });
        }
    }
}

pub fn window() -> eframe::Result<()> {
    // Set the main window configuration options
    let options = NativeOptions {
        maximized: true,
        initial_window_size: Some(egui::Vec2::new(600.0, 300.0)),
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

fn build_default_name() -> String {
    let now = Local::now().to_string();
    format!("screenshot{}", now)
}
