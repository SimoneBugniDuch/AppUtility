use eframe::egui;
use egui::{Modifiers, Key, KeyboardShortcut};
use super::actions::Action;

pub struct AddedShortcut {
    pub modifiers: Modifiers,
    pub key: Option<Key>,
    default: bool,
    pub action: Option<Action>,
    pub description: String,
}

impl AddedShortcut {
    pub fn default() -> Self {
        Self {
            modifiers: Modifiers {
                alt: false,
                ctrl: false,
                shift: false,
                mac_cmd: false,
                command: false,
            },
            key: None,
            default: true,
            action: None,
            description: String::from(""),
        }
    }
}

#[derive(Clone)]
pub struct ShortCut {
    pub name: String,
    pub description: String,
    pub shortcut: KeyboardShortcut,
    active: bool,
    while_viewing_image: bool,
    action: Action,
}

impl ShortCut {
    fn new(modifiers: Modifiers, key: Key, description: String, action: Action) -> Self {
        Self {
            name: action.to_string(),
            description,
            shortcut: KeyboardShortcut { modifiers, key },
            active: true,
            while_viewing_image: action.can_be_performed_during_image_view(),
            action,
        }
    }

    fn shortcut_listener(&self, ctx: &egui::Context) -> Option<Action> {
        if ctx.input_mut(|input_state| input_state.consume_shortcut(&self.shortcut)) && self.active {
            Some(self.action)
        } else {
            None
        }
    }

    fn toggle_active(&mut self) {
        self.active = !self.active;
    }
}

pub struct AllShortcuts {
    pub vec: Vec<ShortCut>,
    pub show: bool,
}

impl AllShortcuts {
    pub fn default() -> Self {
        let mut vec = Vec::new();
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::C, "Copy to clipboard".to_string(), Action::Copy));
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::H, "Go to the home page".to_string(), Action::HomePage));
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::N, "Take a new screenshot".to_string(), Action::NewScreenshot));
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::S, "Save".to_string(), Action::Save));
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::W, "Close the application".to_string(), Action::Close));
        vec.push(ShortCut::new(Modifiers::COMMAND, Key::Z, "Undo".to_string(), Action::Undo));
        Self {
            vec,
            show: false,
        }
    }

    pub fn listener(&self, ctx: &egui::Context, image_viewing: bool) -> Option<Action> {
        for shortcut in self.vec.iter() {
            if shortcut.action == Action::Settings || shortcut.action == Action::Close {
                if shortcut.active {
                    if let Some(action) = shortcut.shortcut_listener(ctx) {
                        return Some(action);
                    }
                }
            } else {
                //Shortcuts that can be pressed when viewing the image
                if shortcut.while_viewing_image == image_viewing && shortcut.active {
                    if let Some(action) = shortcut.shortcut_listener(ctx) {
                        return Some(action);
                    }
                }
            }
        }
        None
    }

    pub fn save_new_shortcut(&mut self, new_shortcut: &mut AddedShortcut) -> Option<ShortCut> {
        if let Some(_) = new_shortcut.action {
            if let Some(_) = new_shortcut.key {
                if !new_shortcut.modifiers.is_none() {
                    let new_sct = ShortCut::new(new_shortcut.modifiers, new_shortcut.key.unwrap(), new_shortcut.description.clone(), new_shortcut.action.unwrap());
                    for sc in self.vec.iter() {
                        if sc.shortcut.eq(&new_sct.shortcut) {
                            return None;
                        }
                    }
                    self.vec.push(new_sct.clone());
                    Some(new_sct.clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove_shortcut(&mut self, shortcut: &mut ShortCut) {
        let mut index = 0;
        for (i, sc) in self.vec.iter().enumerate() {
            if sc.shortcut.eq(&shortcut.shortcut) {
                index = i;
            }
        }
        self.vec.remove(index);
    }

    pub fn toggle_active(&mut self, shortcut: &mut ShortCut) {
        for sc in self.vec.iter_mut() {
            if sc.shortcut.eq(&shortcut.shortcut) {
                sc.toggle_active();
            }
        }
    }
}


pub struct KeyboardKeys {
    pub keys: Vec<Key>,
}
impl KeyboardKeys {
    pub fn default() -> Self {
        Self {
            keys: vec![
                
                Key::A,
                Key::B,
                Key::C,
                Key::D,
                Key::E,
                Key::F,
                Key::G,
                Key::H,
                Key::I,
                Key::J,
                Key::K,
                Key::L,
                Key::M,
                Key::N,
                Key::O,
                Key::P,
                Key::Q,
                Key::R,
                Key::S,
                Key::T,
                Key::U,
                Key::V,
                Key::W,
                Key::X,
                Key::Y,
                Key::Z,
                Key::Num0,
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Num5,
                Key::Num6,
                Key::Num7,
                Key::Num8,
                Key::Num9,
                
                Key::ArrowDown,
                Key::ArrowLeft,
                Key::ArrowRight,
                Key::ArrowUp,
                Key::Escape,
                Key::Tab,
                Key::Backspace,
                Key::Enter,
                Key::Space,
                Key::Insert,
                Key::Delete,
                Key::Home,
                Key::End,
                Key::PageUp,
                Key::PageDown,
                Key::Minus,
                Key::PlusEquals,
                Key::F1,
                Key::F2,
                Key::F3,
                Key::F4,
                Key::F5,
                Key::F6,
                Key::F7,
                Key::F8,
                Key::F9,
                Key::F10,
                Key::F11,
                Key::F12,
                Key::F13,
                Key::F14,
                Key::F15,
                Key::F16,
                Key::F17,
                Key::F18,
                Key::F19,
                Key::F20,
            ],
        }
    }
}