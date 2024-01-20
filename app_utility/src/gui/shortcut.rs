use super::actions::Action;
use eframe::egui;
use egui::{Key, KeyboardShortcut, Modifiers};


#[derive(Clone, PartialEq)]
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
        if ctx.input_mut(|input_state| input_state.consume_shortcut(&self.shortcut)) && self.active
        {
            Some(self.action)
        } else {
            None
        }
    }

    pub fn from_str_to_key(s: &str) -> Option<Key> {
        match s {
            "Down" => Some(Key::ArrowDown),
            "Left" => Some(Key::ArrowLeft),
            "Right" => Some(Key::ArrowRight),
            "Up" => Some(Key::ArrowUp),
            "Escape" => Some(Key::Escape),
            "Tab" => Some(Key::Tab),
            "Backspace" => Some(Key::Backspace),
            "Enter" => Some(Key::Enter),
            "Space" => Some(Key::Space),
            "Insert" => Some(Key::Insert),
            "Delete" => Some(Key::Delete),
            "Home" => Some(Key::Home),
            "End" => Some(Key::End),
            "PageUp" => Some(Key::PageUp),
            "PageDown" => Some(Key::PageDown),
            "Minus" => Some(Key::Minus),
            "Plus" => Some(Key::PlusEquals),
            "0" => Some(Key::Num0),
            "1" => Some(Key::Num1),
            "2" => Some(Key::Num2),
            "3" => Some(Key::Num3),
            "4" => Some(Key::Num4),
            "5" => Some(Key::Num5),
            "6" => Some(Key::Num6),
            "7" => Some(Key::Num7),
            "8" => Some(Key::Num8),
            "9" => Some(Key::Num9),
            "A" => Some(Key::A),
            "B" => Some(Key::B),
            "C" => Some(Key::C),
            "D" => Some(Key::D),
            "E" => Some(Key::E),
            "F" => Some(Key::F),
            "G" => Some(Key::G),
            "H" => Some(Key::H),
            "I" => Some(Key::I),
            "J" => Some(Key::J),
            "K" => Some(Key::K),
            "L" => Some(Key::L),
            "M" => Some(Key::M),
            "N" => Some(Key::N),
            "O" => Some(Key::O),
            "P" => Some(Key::P),
            "Q" => Some(Key::Q),
            "R" => Some(Key::R),
            "S" => Some(Key::S),
            "T" => Some(Key::T),
            "U" => Some(Key::U),
            "V" => Some(Key::V),
            "W" => Some(Key::W),
            "X" => Some(Key::X),
            "Y" => Some(Key::Y),
            "Z" => Some(Key::Z),
            "F1" => Some(Key::F1),
            "F2" => Some(Key::F2),
            "F3" => Some(Key::F3),
            "F4" => Some(Key::F4),
            "F5" => Some(Key::F5),
            "F6" => Some(Key::F6),
            "F7" => Some(Key::F7),
            "F8" => Some(Key::F8),
            "F9" => Some(Key::F9),
            "F10" => Some(Key::F10),
            "F11" => Some(Key::F11),
            "F12" => Some(Key::F12),
            "F13" => Some(Key::F13),
            "F14" => Some(Key::F14),
            "F15" => Some(Key::F15),
            "F16" => Some(Key::F16),
            "F17" => Some(Key::F17),
            "F18" => Some(Key::F18),
            "F19" => Some(Key::F19),
            "F20" => Some(Key::F20),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct AllShortcuts {
    pub vec: Vec<ShortCut>,
    pub show: bool,
    pub all_keys: Vec<String>,
}

impl AllShortcuts {
    pub fn default() -> Self {
        let mut vec = Vec::new();
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::C,
            "Copy to clipboard".to_string(),
            Action::Copy,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::H,
            "Go to the home page".to_string(),
            Action::HomePage,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::N,
            "Take a new screenshot".to_string(),
            Action::NewScreenshot,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::S,
            "Save".to_string(),
            Action::Save,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::W,
            "Close the application".to_string(),
            Action::Close,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::Z,
            "Undo".to_string(),
            Action::Undo,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::K,
            "Manage the timer".to_string(),
            Action::ManageTimer,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::M,
            "Modify the screenshot".to_string(),
            Action::Modify,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::R,
            "Reset the timer".to_string(),
            Action::ResetTimer,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::T,
            "Set the timer".to_string(),
            Action::SetTimer,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::U,
            "Start the timer".to_string(),
            Action::StartTimer,
        ));
        vec.push(ShortCut::new(
            Modifiers::COMMAND,
            Key::Q,
            "Open the settings".to_string(),
            Action::Settings,
        ));

        Self {
            vec,
            show: false,
            all_keys: KeyboardKeys::all_keys(),
        }
    }

    pub fn is_default(shortcut: &AllShortcuts) -> bool {
        let default_shortcuts = AllShortcuts::default();
        for sc in default_shortcuts.vec {
            if !shortcut.vec.contains(&sc) {
                return false;
            }
        }
        false
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

    // Check for duplicate shortcuts
    pub fn has_duplicate_shortcuts(&self) -> bool {
        for (i, shortcut1) in self.vec.iter().enumerate() {
            for shortcut2 in self.vec.iter().skip(i + 1) {
                if shortcut1.shortcut == shortcut2.shortcut {
                    return true;
                }
            }
        }
        false
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

    fn all_keys() -> Vec<String> {
        let mut vec = Vec::new();
        let keys = KeyboardKeys::default().keys;
        for key in keys {
            vec.push(key.name().to_string());
        }
        vec
    }
}
