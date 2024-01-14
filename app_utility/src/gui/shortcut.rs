use egui::{Modifiers, Key, KeyboardShortcut};
use super::actions::Action;

pub struct NewShortcut {
    modifiers: Modifiers,
    key: Option<Key>,
    default: bool,
    action: Option<Action>,
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
            action: None
        }
    }
}

pub struct ShortCut {
    name: String,
    shortcut: KeyboardShortcut,
    active: bool,
    action: Action,
}

impl ShortCut {
    fn new(modifiers: Modifiers, key: Key, action: Action) -> Self {
        Self {
            name: action.to_string(),
            shortcut: KeyboardShortcut { modifiers, key },
            active: true,
            action
        }
    }


}