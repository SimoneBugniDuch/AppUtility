use eframe::egui;
use egui::{Modifiers, Key, KeyboardShortcut};
use super::actions::Action;

pub struct NewShortcut {
    modifiers: Modifiers,
    key: Option<Key>,
    default: bool,
    action: Option<Action>,
    description: String,
}

impl NewShortcut {
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
}

pub struct AllShortcuts {
    pub vec: Vec<ShortCut>,
    pub show: bool,
}

impl AllShortcuts {
    pub fn default() -> Self {
        let mut vec = Vec::new();
        vec.push(ShortCut::new(Modifiers::CTRL, Key::C, "Copy to clipboard".to_string(), Action::Copy));
        vec.push(ShortCut::new(Modifiers::CTRL, Key::H, "Go to the home page".to_string(), Action::HomePage));
        vec.push(ShortCut::new(Modifiers::CTRL, Key::N, "Take a new screenshot".to_string(), Action::NewScreenshot));
        vec.push(ShortCut::new(Modifiers::CTRL, Key::S, "Save".to_string(), Action::Save));
        vec.push(ShortCut::new(Modifiers::CTRL, Key::W, "Close the application".to_string(), Action::Close));
        vec.push(ShortCut::new(Modifiers::CTRL, Key::Z, "Undo".to_string(), Action::Undo));
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
}