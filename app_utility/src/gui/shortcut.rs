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
            description: String::new(),
        }
    }
}

pub struct ShortCut {
    name: String,
    description: String,
    shortcut: KeyboardShortcut,
    active: bool,
    action: Action,
}

impl ShortCut {
    fn new(modifiers: Modifiers, key: Key, description: String, action: Action) -> Self {
        Self {
            name: action.to_string(),
            description,
            shortcut: KeyboardShortcut { modifiers, key },
            active: true,
            action
        }
    }
}

pub struct ShortcutVec {
    set: Vec<ShortCut>,
    pub show: bool,
}

impl ShortcutVec {
    pub fn default() -> Self {
        let mut set = Vec::new();
        set.push(ShortCut::new(Modifiers::COMMAND, Key::Z, "Undo".to_string(), Action::Undo));
        set.push(ShortCut::new(Modifiers::COMMAND, Key::W, "Close the application".to_string(), Action::Close));
        Self {
            set,
            show: false,
        }
    }
}