mod actions;
mod screenshots;
mod shortcut;

use std::{sync::Arc, time::Duration};

use chrono::Local;
use eframe::{
    egui::{self, text, Color32, Layout, Sense, TextureHandle, Visuals, Window},
    epaint::vec2,
    run_native, App, Frame,
};
use image::{self, load_from_memory, ImageError};

use self::actions::Action;
use self::screenshots::Screenshots;
use self::shortcut::NewShortcut;

struct AppUtility {
    rectangle: Rectangle,
    default_path: String,
    new_shortcut: NewShortcut,
    default_name: String,
    default_name_selected: bool,
    default_number: usize,
    hide: bool,
    selection_mode: Selection,
    screenshots: Screenshots,
    buffer: Option<Vec<u8>>,
    view_image: bool,
    texture: Option<TextureHandle>,
    selecting_area: bool,
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

enum Selection {
    Fullscreen,
    Area,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::light());
        Self {
            rectangle: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            default_path: "screenshot".to_string(),
            new_shortcut: NewShortcut::default(),
            default_name: build_default_name(),
            default_name_selected: true,
            default_number: 0,
            hide: false,
            selection_mode: Selection::Fullscreen,
            selecting_area: false,
            screenshots: Screenshots::new(),
            buffer: None,
            view_image: false,
            texture: None,
        }
    }

    fn make_action(&mut self, action: Action, ctx: &egui::Context, frame: &mut Frame) {
        match action {
            Action::Capture => {
                self.hide = true;
                frame.set_visible(false);
                println!("Running the action");
            }
            Action::Close => {
                frame.close();
            }
            Action::Copy => {}
            Action::Modify => {}
            Action::NewScreenshot => {}
            Action::Save => {
                let mut filename = build_default_name();
                if !self.default_name_selected {
                    if self.default_number != 0 {
                        filename = format!("{}_{}", self.default_name, self.default_number);
                        self.default_number += 1;
                    } else {
                        filename = self.default_name.clone();
                        self.default_number += 1;
                    }
                }
                
            },
            Action::SelectArea => {
                self.selection_mode = Selection::Area;
                self.selecting_area = true;
            }
            Action::SelectFullscreen => {
                self.selection_mode = Selection::Fullscreen;
                self.selecting_area = false;
            }
            Action::Undo => {}
        }
    }
}

impl App for AppUtility {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        if self.hide {
            println!("Now I'm hiding");
            std::thread::sleep(Duration::from_millis(300));
            let mut screen = self.screenshots.get_screen();
            let img;
            match self.selection_mode {
                Selection::Area => {
                    img = screen
                        .capture_area(
                            self.rectangle.x.floor() as i32,
                            self.rectangle.y.floor() as i32,
                            self.rectangle.width.floor() as u32,
                            self.rectangle.height.floor() as u32,
                        )
                        .unwrap();
                    println!("Capturing area screen!");
                }
                Selection::Fullscreen => {
                    img = screen.capture().unwrap();
                    println!("Capturing screen!");
                }
            }
            self.buffer = Some(img.to_png(None).unwrap());
            self.texture = Some(ctx.load_texture(
                "new_image",
                load_image_from_mem(&self.buffer.clone().unwrap()).unwrap(),
                Default::default(),
            ));
            self.hide = false;
            self.view_image = true;
            frame.set_visible(true);
        }

        Window::new("menu bar")
            .title_bar(false)
            .movable(!self.view_image)
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .default_rect(egui::Rect::from_center_size(
                egui::Pos2::new(frame.info().window_info.size.x / 2.0 - 70.0, 30.0),
                egui::Vec2::new(316.0, 30.0),
            ))
            .fixed_size([400.0, 50.0])
            .open(&mut (!self.view_image && !self.selecting_area))
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout {
                        main_dir: egui::Direction::LeftToRight,
                        main_align: egui::Align::Center,
                        main_wrap: false,
                        main_justify: false,
                        cross_align: egui::Align::Center,
                        cross_justify: true,
                    },
                    |ui| {
                        if !self.view_image {
                            if custom_button(
                                ui,
                                "🖵 Fullscreen shot",
                                egui::Color32::BLACK,
                                egui::Color32::LIGHT_BLUE,
                            )
                            .on_hover_text("Take a screenshot of the entire screen")
                            .clicked()
                            {
                                self.make_action(Action::SelectFullscreen, ctx, frame);
                                println!("Capture clicked");
                                self.make_action(Action::Capture, ctx, frame);
                            }
                            if custom_button(
                                ui,
                                "⛶ Area shot",
                                egui::Color32::WHITE,
                                egui::Color32::GREEN,
                            )
                            .on_hover_text("Take a screenshot of an area")
                            .clicked()
                            {
                                self.make_action(Action::SelectArea, ctx, frame);
                                println!("You want an area shot?");
                            }

                            if custom_button(
                                ui,
                                "🔧 SETTINGS",
                                egui::Color32::WHITE,
                                egui::Color32::RED,
                            )
                            .on_hover_text("window that contains the settings")
                            .clicked()
                            {
                                // Your SETTINGS button logic
                            }

                            if circular_button(
                                ui,
                                " x ",
                                egui::Color32::WHITE,
                                egui::Color32::RED,
                                20.0,
                            )
                            .on_hover_text("Close the app")
                            .clicked()
                            {
                                self.make_action(Action::Close, ctx, frame);
                            }
                        }
                    },
                )
            });

        Window::new("Screenshot taken")
            .title_bar(false)
            .open(&mut self.view_image.clone())
            .frame(egui::Frame {
                ..Default::default()
            })
            .fixed_size([600.0, 50.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout {
                        main_dir: egui::Direction::LeftToRight,
                        main_align: egui::Align::Center,
                        main_wrap: false,
                        main_justify: false,
                        cross_align: egui::Align::Center,
                        cross_justify: true
                    }, |ui| {
                        if self.view_image {
                            println!("Now I'm seeing the image");
                        }
                    })
            });

        Window::new("View screenshot")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame {
                ..Default::default()
            })
            .fixed_size([1200.0, 600.0])
            .resizable(false)
            .open(&mut self.view_image)
            .show(ctx, |ui| {
                let dim_img = resize_to_fit_container(
                    frame.info().window_info.size.x,
                    frame.info().window_info.size.y,
                    self.texture.clone().unwrap().size_vec2()[0],
                    self.texture.clone().unwrap().size_vec2()[1],
                );
                let (mut response, painter) =
                    ui.allocate_painter(vec2(dim_img.0, dim_img.1), Sense::drag());
                painter.image(
                    self.texture.clone().unwrap().id(),
                    egui::Rect::from_center_size(
                        egui::Pos2::new(
                            (frame.info().window_info.size[0]) / 2.0,
                            (frame.info().window_info.size[1]) / 2.0,
                        ),
                        egui::Vec2::new(dim_img.0, dim_img.1),
                    ),
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                println!("I'm viewing the image!!");
            });

        let window = Window::new("Select area")
            .title_bar(false)
            .default_size(egui::vec2(300.0, 300.0))
            .resizable(true)
            .movable(true)
            .resize(|r| {
                r.max_size(egui::vec2(
                    frame.info().window_info.size[0],
                    frame.info().window_info.size[1],
                ))
            })
            .resize(|r| r.min_size(egui::vec2(2.0, 2.0)))
            .frame(egui::Frame { 
                ..Default::default() 
            })
            .open(&mut self.selecting_area)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                println!("Am I here?!");
            });

        if self.selecting_area {
            println!("Do I need to be here?");
            let rect = window.unwrap().response.rect;
            let mut corr = 1.0 ;
            if cfg!(target_os = "windows") {
                corr = frame.info().native_pixels_per_point.unwrap();
            }
            self.rectangle = Rectangle {
                x: rect.left() * corr,
                y: rect.top() * corr,
                width: rect.width() * corr,
                height: rect.height() * corr,
            }
        }
    }
}

fn custom_button(
    ui: &mut egui::Ui,
    text: &str,
    text_color: Color32,
    bg_color: Color32,
) -> egui::Response {
    // Store the previous button style
    let previous_button_padding = ui.style().spacing.button_padding;

    // Set the desired button padding
    // let padding = egui::vec2(30.0, 10.0);

    let font_size = 20.0;
    // Create a RichText with the desired text color, bold style, and font size
    let rich_text = egui::RichText::new(text)
        .color(text_color)
        .size(font_size) // Set the font size
        .strong();

    let button_size = egui::vec2(text.len() as f32 * 10.0, font_size);

    // Create and add the button to the UI
    let button = egui::Button::new(rich_text).fill(bg_color).rounding(10.0); // Set the rounding for corners

    let response = ui.add_sized(button_size, button);

    // Reset the button padding to previous
    ui.style_mut().spacing.button_padding = previous_button_padding;

    response
}

fn circular_button(
    ui: &mut egui::Ui,
    text: &str,
    text_color: Color32,
    bg_color: Color32,
    radius: f32,
) -> egui::Response {
    let desired_size = egui::Vec2::splat(radius * 2.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.painter().circle_filled(rect.center(), radius, bg_color);

        // Create a FontId with the specified font size
        let font_id = egui::FontId::new(20.0, egui::FontFamily::Proportional);

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font_id,
            text_color,
        );
    }

    response
}

pub fn window() -> eframe::Result<()> {
    // Set the main window configuration options
    let options = eframe::NativeOptions {
        maximized: true,
        decorated: false,
        transparent: true,
        resizable: false,
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

fn resize_to_fit_container(
    container_width: f32,
    container_height: f32,
    image_width: f32,
    image_height: f32,
) -> (f32, f32) {
    let container_ratio = container_width / container_height;
    let image_ratio = image_width / image_height;

    if container_ratio > image_ratio {
        let new_height = container_height;
        let new_width = new_height * image_ratio;
        (new_width, new_height)
    } else {
        let new_width = container_width;
        let new_height = new_width / image_ratio;
        (new_width, new_height)
    }
}

fn load_image_from_mem(image_data: &[u8]) -> Result<egui::ColorImage, ImageError> {
    let image = load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
