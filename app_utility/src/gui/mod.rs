mod actions;
mod screenshots;
mod shortcut;
mod timer;

use arboard::{Clipboard, ImageData};
use std::{time::{Duration, Instant}, fs, borrow::Cow};
use native_dialog::FileDialog;
use chrono::Local;
use eframe::{
    egui::{self, Color32, Layout, Sense, TextureHandle, Visuals, Window},
    epaint::vec2,
    run_native, App, Frame,
};
use image::{self, load_from_memory, ImageError};

use self::{screenshots::Screenshots, timer::Timer, actions::Action, shortcut::{AllShortcuts, NewShortcut}};

struct AppUtility {
    buffer: Option<Vec<u8>>,
    default_name: String,
    default_name_selected: bool,
    default_number: usize,
    default_path: String,
    hide: bool,
    modification: bool,
    modifications_vector: Vec<Modifier>,
    modified_element: ModifiedElement,
    modifier: Modifier,
    new_shortcut: NewShortcut,
    rectangle: Rectangle,
    screenshots: Screenshots,
    selecting_area: bool,
    selection_mode: Selection,
    shortcuts: AllShortcuts,
    show_settings: bool,
    texture: Option<TextureHandle>,
    timer: Timer,
    view_image: bool,
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

#[derive(PartialEq, Debug)]
enum Modifier {
    NotSelected,
    Pen,
    Rect,
    Arrow,
    Text,
    Crop,
    Line,
    Circle,
}

struct ModifiedElement {
    stroke: egui::Stroke,
    pen: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    rect: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    circle: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    arrow: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    line: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    text: String,
}

impl AppUtility {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::light());
        Self {
            buffer: None,
            default_name: build_default_name(),
            default_name_selected: true,
            default_number: 0,
            default_path: "screenshot".to_string(),
            hide: false,
            modification: false,
            modifications_vector: Default::default(),
            modifier: Modifier::NotSelected,
            modified_element: ModifiedElement {
                stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                pen: Default::default(),
                rect: Default::default(),
                circle: Default::default(),
                arrow: Default::default(),
                line: Default::default(),
                text: "Example".to_owned(),
            },
            new_shortcut: NewShortcut::default(),
            rectangle: Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
            screenshots: Screenshots::new(),
            selecting_area: false,
            selection_mode: Selection::Fullscreen,
            shortcuts: AllShortcuts::default(),
            show_settings: false,
            texture: None,
            timer: Timer::new(),
            view_image: false,
        }
    }

    fn make_action(&mut self, action: Action, ctx: &egui::Context, frame: &mut Frame) {
        match action {
            Action::Capture => {
                self.hide = true;
                self.timer.reset_timer();
                frame.set_visible(false);
            }
            Action::Close => {
                frame.close();
            }
            Action::Copy => {
                let mut clipboard = Clipboard::new().unwrap();
                let image = load_image_from_mem(&self.buffer.clone().unwrap()).unwrap();
                let bytes = image.as_raw();
                let image_data = ImageData {
                    width: image.width() as usize,
                    height: image.height() as usize,
                    bytes: Cow::from(bytes.as_ref()),
                };
                clipboard.set_image(image_data).unwrap();
            }
            Action::HomePage => {
                self.selecting_area = false;
                self.view_image = false;
                self.show_settings = false;
            }
            Action::ManageTimer => {
                let now = Instant::now();
                if now.duration_since(self.timer.start_instant.unwrap()).as_secs_f32() >= 1.0 {
                    self.timer.decrement_timer();
                    if self.timer.seconds == 0 {
                        self.timer.reset_timer();
                        self.hide = true;
                        frame.set_visible(false);
                    }
                    self.timer.start_instant = Some(now);
                }
                ctx.request_repaint();
            }
            Action::Modify => {
                self.modification = true;
            }
            Action::NewScreenshot => {
                self.hide = false;
                self.view_image = false;
                self.selection_mode = Selection::Fullscreen;
                self.selecting_area = false;
                self.show_settings = false;
            }
            Action::ResetTimer => {
                self.timer.reset_timer();
            }
            Action::Save => {
                let mut filename = build_default_name();
                if self.default_name_selected {
                    if self.default_number != 0 {
                        filename = format!("{}_{}", self.default_name, self.default_number);
                        self.default_number += 1;
                    } else {
                        filename = self.default_name.clone();
                        self.default_number += 1;
                    }
                }
                let mut dir = std::env::current_dir().unwrap();
                dir.push(&self.default_path);
                if !dir.exists() {
                    dir = std::env::current_dir().unwrap();
                    dir.push("screenshots");
                }
                let res = match FileDialog::new()
                    .set_location(&dir)
                    .set_filename(&filename)
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .add_filter("GIF Image", &["gif"])
                    .show_save_single_file()
                {
                    Ok(res) => res,
                    Err(_) => FileDialog::new()
                        .set_location("~")
                        .set_filename(&filename)
                        .add_filter("PNG Image", &["png"])
                        .add_filter("JPEG Image", &["jpg", "jpeg"])
                        .add_filter("GIF Image", &["gif"])
                        .show_save_single_file()
                        .unwrap(),
                };
                match res {
                    Some(res) => fs::write(res.clone(), self.buffer.clone().unwrap()).unwrap(),
                    None => {}
                }
            }
            Action::SelectArea => {
                self.selection_mode = Selection::Area;
                self.selecting_area = true;
            }
            Action::SelectFullscreen => {
                self.selection_mode = Selection::Fullscreen;
                self.selecting_area = false;
            }
            Action::Settings => {
                self.show_settings = true;
            }
            Action::SetTimer => {
                self.timer.open_form();
            }
            Action::StartTimer => {
                if self.timer.seconds > 0 {
                    self.timer.start_timer();
                } else {
                    // self.make_action(Action::SetTimer, ctx, frame);
                    self.make_action(Action::Capture, ctx, frame);
                }
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
            self.selecting_area = false;
            frame.set_visible(true);
        }

        Window::new("home_page menu_bar")
            .title_bar(false)
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .default_pos(calc_center_position(ctx, 500.0, 20.0))
            .default_size(egui::vec2(250.0, 30.0))
            .resizable(false)
            .open(&mut (!self.view_image && !self.selecting_area && !self.show_settings && !self.timer.form_opened() && !self.timer.is_running()))
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
                        match self.shortcuts.listener(ctx, self.view_image) {
                            Some(action) => self.make_action(action, ctx, frame),
                            None => {}
                        }

                        if !self.view_image {
                            // if self.timer.is_running() {
                            //     self.make_action(Action::ManageTimer, ctx, frame);                               
                            // }

                            if custom_button(
                                ui,
                                "📷  Fullscreen shot",
                                egui::Color32::WHITE,
                                egui::Color32::from_rgb(114, 134, 211),
                            )
                            .on_hover_text("Take a screenshot of the entire screen")
                            .clicked()
                            {
                                self.make_action(Action::SelectFullscreen, ctx, frame);
                                println!("Capture clicked");
                                self.make_action(Action::StartTimer, ctx, frame);
                            }

                            ui.add_space(10.0);
                            if custom_button(
                                ui,
                                "⛶  Area shot",
                                egui::Color32::WHITE,
                                egui::Color32::from_rgb(142, 167, 233),
                            )
                            .on_hover_text("Take a screenshot of an area")
                            .clicked()
                            {
                                self.make_action(Action::SelectArea, ctx, frame);
                                println!("You want an area shot?");
                            }

                            ui.add_space(10.0);
                            if custom_button(
                                ui, 
                                "TIMER", 
                                egui::Color32::DARK_GRAY, 
                                egui::Color32::YELLOW)
                            .on_hover_text("Take a screenshot after a delay")
                            .clicked() {
                                self.make_action(Action::SetTimer, ctx, frame);
                            }
                            ui.label(format!("Actual delay: {}", self.timer.seconds));

                            ui.add_space(10.0);
                            if custom_button(
                                ui,
                                "🔧  SETTINGS",
                                egui::Color32::DARK_GRAY,
                                egui::Color32::from_rgb(229, 224, 255),
                            )
                            .on_hover_text("Open the settings menu")
                            .clicked()
                            {
                                self.make_action(Action::Settings, ctx, frame);
                            }

                            ui.add_space(10.0);
                            if circular_button(
                                ui,
                                " x ",
                                egui::Color32::WHITE,
                                egui::Color32::from_rgb(210, 69, 69),
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

        Window::new("screenshot_taken toolbar")
            //TODO: QUI BISOGNA INSERIRE I BOTTONI DI MODIFICA, DI COPIA ECC...
            .title_bar(false)
            .open(&mut self.view_image.clone())
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .fixed_size([600.0, 30.0])
            .default_pos(calc_center_position(ctx, 450.0, 20.0))
            .resizable(false)
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
                        match self.shortcuts.listener(ctx, self.view_image) {
                            Some(action) => self.make_action(action, ctx, frame),
                            None => {}
                        }

                        if self.view_image && !self.modification {
                            println!("Now I'm seeing the image");
                            if custom_button(ui, "Modify", Color32::BLACK, Color32::GRAY).clicked()
                            {
                                self.make_action(Action::Modify, ctx, frame);
                            }
                            if custom_button(ui, "Save", Color32::BLACK, Color32::GRAY).clicked() {
                                self.make_action(Action::Save, ctx, frame);
                            }
                            if custom_button(ui, "Copy", Color32::BLACK, Color32::GRAY).clicked() {
                                self.make_action(Action::Copy, ctx, frame);
                            }
                            if custom_button(ui, "New screenshot", Color32::BLACK, Color32::GRAY)
                                .clicked()
                            {
                                self.make_action(Action::NewScreenshot, ctx, frame);
                                //Magari se ci sono delle modifiche non salvate conviene chiedere conferma (Discard all unsaved changes?)
                            }
                            if custom_button(ui, "Settings", Color32::BLACK, Color32::GRAY)
                                .clicked()
                            {
                                self.make_action(Action::Settings, ctx, frame);
                            }
                            if circular_button(
                                ui,
                                " x ",
                                egui::Color32::WHITE,
                                egui::Color32::from_rgb(210, 69, 69),
                                15.0,
                            )
                            .on_hover_text("Close the app")
                            .clicked()
                            {
                                self.make_action(Action::Close, ctx, frame);
                            }
                        } else {
                            if ui.button(" 🖊  ").on_hover_text("Draw").clicked() {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Pen;
                            }
                            if ui.button("  /  ").on_hover_text("Draw a line").clicked() {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Line;
                            }
                            if ui.button("  ↖  ").on_hover_text("Draw an arrow").clicked() {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Arrow;
                            }
                            if ui
                                .button("  ☐  ")
                                .on_hover_text("Draw a rectangle")
                                .clicked()
                            {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Rect;
                            }
                            if ui.button("  ⭕  ").on_hover_text("Draw a circle").clicked() {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Circle;
                            }
                            if ui
                                .button("  Text  ")
                                .on_hover_text("Write a text")
                                .clicked()
                            {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Text;
                            }
                            if self.modifier == Modifier::Text {
                                egui::ScrollArea::vertical().min_scrolled_height(30.0).show(
                                    ui,
                                    |ui| {
                                        ui.add(
                                            egui::TextEdit::multiline(
                                                &mut self.modified_element.text,
                                            )
                                            .desired_rows(1)
                                            .desired_width(500.0),
                                        );
                                    },
                                );

                                if ui.button("  Save text  ").clicked() {
                                    self.modifications_vector.push(Modifier::Text);
                                };
                            }
                            if ui.button("  ⛶  ").on_hover_text("Crop").clicked() {
                                //fare la modifica effettiva
                                self.modifier = Modifier::Crop;
                            }
                            if self.modifier == Modifier::Crop {
                                if ui.button("  Save crop  ").clicked() {
                                    self.modifier = Modifier::NotSelected;
                                    self.hide = true;
                                }
                            }

                            if ui.button("  ⟲  ").on_hover_text("undo").clicked() {
                                self.make_action(Action::Undo, ctx, frame);
                            }
                            if ui.button("  X  ").on_hover_text("Close").clicked() {
                                self.modification = false;
                            }
                        }
                    },
                )
            });

        Window::new("screenshot_view")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame {
                fill: egui::Color32::BLACK,
                inner_margin: egui::Margin::same(10.0),
                ..Default::default()
            })
            .fixed_size([1200.0, 600.0])
            .resizable(false)
            .open(&mut self.view_image)
            .show(ctx, |ui| {
                let dim_img = resize_to_fit_container(
                    frame.info().window_info.size.x / 3.0 * 2.0,
                    frame.info().window_info.size.y / 3.0 * 2.0,
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
                //qua vedo e salvo le modifiche che sto effettuando dopo aver schiacciato il bottone
                if self.modification {
                    match self.modifier {
                        Modifier::NotSelected => {}
                        Modifier::Pen => {
                            //azione
                            response.clone().on_hover_cursor(egui::output::CursorIcon::PointingHand);
                            if self.modified_element.pen.is_empty() {
                                self.modified_element.pen.push(vec![]);
                            }
                            let current_line = self.modified_element.pen.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.modified_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.modified_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.modified_element.pen.push(vec![]);
                                response.mark_changed();
                                self.modifications_vector.push(Modifier::Pen);
                            }
                        }
                        Modifier::Rect => {
                            //azione
                            response.clone().on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.modified_element.rect.is_empty() {
                                self.modified_element.rect.push(vec![]);
                            }
                            let current_line = self.modified_element.rect.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.modified_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.modified_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.modified_element.rect.push(vec![]);
                                response.mark_changed();
                                self.modifications_vector.push(Modifier::Rect);
                            }
                        }
                        Modifier::Arrow => {
                            //azione
                            response.clone().on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.modified_element.arrow.is_empty() {
                                self.modified_element.arrow.push(vec![]);
                            }
                            let current_line = self.modified_element.arrow.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.modified_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.modified_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.modified_element.arrow.push(vec![]);
                                response.mark_changed();
                                self.modifications_vector.push(Modifier::Arrow);
                            }
                        }
                        Modifier::Line => {
                            //azione
                            response.clone().on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.modified_element.line.is_empty() {
                                self.modified_element.line.push(vec![]);
                            }

                            let current_line = self.modified_element.line.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.modified_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.modified_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.modified_element.line.push(vec![]);
                                response.mark_changed();
                                self.modifications_vector.push(Modifier::Line);
                            }
                        }
                        Modifier::Circle => {
                            //azione
                            response.clone().on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.modified_element.circle.is_empty() {
                                self.modified_element.circle.push(vec![]);
                            }
                            let current_line = self.modified_element.circle.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.modified_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.modified_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.modified_element.circle.push(vec![]);
                                response.mark_changed();
                                self.modifications_vector.push(Modifier::Circle);
                            }
                        }
                        Modifier::Text => {
                            //azione
                            let res = egui::Area::new("text")
                                .movable(true)
                                .default_pos(egui::Pos2::new(
                                    (frame.info().window_info.size[0] - 20.0) / 2.0,
                                    (frame.info().window_info.size[1] - 20.0) / 2.0,
                                ))
                                .drag_bounds(egui::Rect::from_center_size(
                                    egui::Pos2::new(
                                        (frame.info().window_info.size[0]) / 2.0,
                                        (frame.info().window_info.size[1]) / 2.0,
                                    ),
                                    egui::Vec2::new(dim_img.0, dim_img.1),
                                ))
                                .order(egui::layers::Order::Foreground)
                                .show(ctx, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{}",
                                                self.modified_element.text,
                                            ))
                                            .color(self.modified_element.stroke.color)
                                            .size(
                                                self.modified_element.stroke.width * 20.0 + 0.1,
                                            ),
                                        );
                                    });
                                });
                        }
                        Modifier::Crop => {
                            let area = egui::Window::new("crop_area")
                                .title_bar(false)
                                .default_size(egui::vec2(320.0, 240.0))
                                .resizable(true)
                                .movable(true)
                                .resize(|r| r.min_size(egui::vec2(1.0, 1.0)))
                                .resize(|r| r.max_size(egui::vec2(dim_img.0, dim_img.1)))
                                .default_pos(egui::Pos2::new(
                                    (frame.info().window_info.size[0] - 320.0) / 2.0,
                                    (frame.info().window_info.size[1] - 240.0) / 2.0,
                                ))
                                .drag_bounds(egui::Rect::from_center_size(
                                    egui::Pos2::new(
                                        (frame.info().window_info.size[0]) / 2.0,
                                        (frame.info().window_info.size[1]) / 2.0,
                                    ),
                                    egui::Vec2::new(dim_img.0, dim_img.1),
                                ))
                                .frame(egui::Frame {
                                    stroke: egui::Stroke::new(1.5, egui::Color32::WHITE),
                                    shadow: egui::epaint::Shadow::small_light(),
                                    ..Default::default()
                                })
                                .show(ctx, |ui| {
                                    ui.allocate_space(ui.available_size());
                                });

                            let rectangle = area.unwrap().response.rect;
                            let mut adj = 1.0;
                            if cfg!(target_os = "windows") {
                                adj = frame.info().native_pixels_per_point.unwrap();
                            } 
                            self.rectangle = Rectangle {
                                x: (rectangle.left()) * adj,
                                y: (rectangle.top()) * adj,
                                width: rectangle.width() * adj,
                                height: rectangle.height() * adj,
                            };
                        }

                        _ => {}
                    }
                }
                let pen = self.modified_element.pen
                    .iter().filter(|line| line.len() >= 2)
                    .map(|line| {
                        let points: Vec<egui::Pos2> = line.iter().map(|p| p.0).collect();
                        let stroke = line[0].1;
                        egui::Shape::line(points, stroke)
                    });
                let rect = self.modified_element.rect
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let rect = egui::Rect::from_two_pos(
                            line.first().unwrap().0,
                            line.last().unwrap().0,
                        );
                        egui::Shape::rect_stroke(rect, egui::Rounding::none(), line[0].1)
                    });
                let circle = self.modified_element.circle
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        egui::Shape::circle_stroke(
                            line.first().unwrap().0,
                            line.first().unwrap().0.distance(line.last().unwrap().0),
                            line[0].1,
                        )
                    });
                let line = self.modified_element.line
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let vec = [line.first().unwrap().0, line.last().unwrap().0];
                        egui::Shape::line_segment(vec, line[0].1)
                    });

                for element in self.modified_element.arrow.clone() {
                    if element.first().is_some() && element.last().is_some() {
                        let line = element.first().unwrap().0 - element.last().unwrap().0;
                        painter.arrow(element.first().unwrap().0, -line, element[0].1);
                    }
                }

                painter.extend(pen);
                painter.extend(line);
                painter.extend(rect);
                painter.extend(circle);
            });

        Window::new("screenshot_area menu_bar")
            .title_bar(false)
            .frame(egui::Frame {
                fill: egui::Color32::LIGHT_GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .default_size(egui::vec2(200.0, 30.0))
            .default_pos(calc_center_position(ctx, 300.0, 30.0))
            .resizable(false)
            .open(&mut (self.selecting_area.clone() && !self.timer.form_opened() && !self.timer.is_running()))
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
                        if custom_button(
                            ui,
                            " 📷  Capture  ",
                            Color32::WHITE,
                            Color32::from_rgb(142, 167, 233),
                        )
                        .on_hover_text("Capture the area screenshot!")
                        .clicked()
                        {
                            self.make_action(Action::StartTimer, ctx, frame);
                        }

                        ui.add_space(10.0);
                        if custom_button(
                            ui, 
                            "TIMER", 
                            egui::Color32::DARK_GRAY, 
                            egui::Color32::YELLOW)
                        .on_hover_text("Take a screenshot after a delay")
                        .clicked() {
                            self.make_action(Action::SetTimer, ctx, frame);
                        }
                        ui.label(format!("Actual delay: {}", self.timer.seconds));

                        ui.add_space(10.0);
                        if custom_button(
                            ui,
                            " ⟲  HomePage  ",
                            Color32::WHITE,
                            Color32::from_rgb(210, 69, 69),
                        )
                        .on_hover_text("Go back to the homepage")
                        .clicked()
                        {
                            self.make_action(Action::HomePage, ctx, frame);
                        }
                    },
                )
            });

        let window = Window::new("selection_area_rectangle")
            .title_bar(false)
            .default_size(egui::vec2(500.0, 300.0))
            .resizable(true)
            .movable(true)
            .default_pos(egui::Pos2::new(
                (frame.info().window_info.size[0] - 500.0) / 2.0,
                (frame.info().window_info.size[1] - 300.0) / 2.0,
            ))
            .resize(|r| {
                r.max_size(egui::vec2(
                    frame.info().window_info.size[0],
                    frame.info().window_info.size[1],
                ))
            })
            .resize(|r| r.min_size(egui::vec2(2.0, 2.0)))
            .frame(egui::Frame {
                stroke: egui::Stroke {
                    width: 1.0,
                    color: Color32::WHITE,
                },
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
            let mut corr = 1.0;
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

        Window::new("Settings")
            .open(&mut self.show_settings)
            .frame(egui::Frame {
                fill: egui::Color32::LIGHT_GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .default_pos(calc_center_position(ctx, 800.0, 100.0))
            .movable(true)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading("Shortcuts settings: ");
                ui.separator();
                ui.add_space(10.0);
                egui::Grid::new("shortcut_grid")
                    .spacing([170.0, 5.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Shortcut Name");
                        ui.label("Description");
                        ui.label("Keyboard combination");
                        ui.end_row();

                        for shortcut in &mut self.shortcuts.vec {
                            let shortcut::ShortCut {
                                name,
                                description,
                                shortcut,
                                ..
                            } = shortcut;

                            // Calculate width based on text length plus padding
                            let name_width = 100.0;
                            let description_width = 300.0;

                            ui.add_sized([name_width, 0.0], egui::TextEdit::singleline(name));
                            ui.add_sized(
                                [description_width, 0.0],
                                egui::TextEdit::singleline(description),
                            );
                            ui.add_sized(
                                [name_width, 0.0],
                                egui::TextEdit::singleline(&mut ctx.format_shortcut(&shortcut)),
                            );

                            ui.end_row();
                        }
                    });

                // Buttons for saving and discarding changes
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if custom_button_with_font_size(
                        ui,
                        "Save all changes",
                        egui::Color32::DARK_GRAY,
                        egui::Color32::from_rgb(229, 224, 255),
                        10.0,
                    )
                    .clicked()
                    {
                        // Implement logic to discard changes and revert to original shortcuts
                    }
                    ui.add_space(10.0);
                    if custom_button_with_font_size(
                        ui,
                        " Discard all changes",
                        egui::Color32::DARK_GRAY,
                        egui::Color32::from_rgb(229, 224, 255),
                        10.0,
                    )
                    .clicked()
                    {
                        // Implement logic to discard changes and revert to original shortcuts
                    }
                });
                // End of shortcuts settings window
            });

        Window::new("Timer form")
            .title_bar(false)
            .open(&mut self.timer.form_opened())
            .movable(true)
            .resizable(false)
            .frame(egui::Frame {
                fill: egui::Color32::LIGHT_GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .default_pos(calc_center_position(ctx, 800.0, 100.0))
            .show(ctx, |ui| {
                ui.label("Timer (in seconds)");
                ui.add(egui::DragValue::new(&mut self.timer.seconds).clamp_range(0..=60));

                if custom_button(ui, "Save changes",egui::Color32::DARK_GRAY, egui::Color32::YELLOW).clicked() {
                    self.timer.close_form();
                }

                if custom_button(ui, "Reset", egui::Color32::DARK_GRAY, egui::Color32::YELLOW).clicked() {
                    self.make_action(Action::ResetTimer, ctx, frame);
                }
            });

        Window::new("Timer running")
            .title_bar(false)
            .open(&mut (self.timer.is_running() && !self.hide))
            .movable(false)
            .resizable(false)
            .default_size(egui::vec2(100.0, 100.0))
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame {
                fill: egui::Color32::TRANSPARENT,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(self.timer.seconds.to_string()).size(30.0).color(egui::Color32::GOLD));
                if self.timer.is_running() {
                    self.make_action(Action::ManageTimer, ctx, frame);                               
                }
                if custom_button(ui, "Cancel", egui::Color32::DARK_GRAY, egui::Color32::YELLOW).clicked() {
                    self.make_action(Action::ResetTimer, ctx, frame);
                } 
            });
    }
}

fn calc_center_position(ctx: &egui::Context, window_width: f32, pos_y: f32, ) -> egui::Pos2 {
    let screen_width = ctx.available_rect().width(); // Set the screen width
    let window_x = (screen_width - window_width) / 2.0; // Centered x-coordinate
    egui::pos2(window_x, pos_y) // Return the position
}

// Version with font_size
fn custom_button_with_font_size(
    ui: &mut egui::Ui,
    text: &str,
    text_color: Color32,
    bg_color: Color32,
    font_size: f32,
) -> egui::Response {
    // Store the previous button style
    let previous_button_padding = ui.style().spacing.button_padding;

    // Create a RichText with the desired text color, bold style, and font size
    let rich_text = egui::RichText::new(text)
        .color(text_color)
        .size(font_size)
        .strong();

    let button_size = egui::vec2(text.len() as f32 * 8.0, font_size);

    // Create and add the button to the UI
    let button = egui::Button::new(rich_text).fill(bg_color).rounding(10.0);

    let response = ui.add_sized(button_size, button);

    // Reset the button padding to previous
    ui.style_mut().spacing.button_padding = previous_button_padding;

    response
}

// Version without font_size
fn custom_button(
    ui: &mut egui::Ui,
    text: &str,
    text_color: Color32,
    bg_color: Color32,
) -> egui::Response {
    custom_button_with_font_size(ui, text, text_color, bg_color, 13.0)
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

fn build_default_name() -> String {
    let now = Local::now()
        .to_string()
        .replace("-", "")
        .replace(":", "_")
        .replace(" ", "-");
    format!("Screenshot_{}", now)[..28].to_string()
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
