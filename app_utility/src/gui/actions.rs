pub enum Action {
    Capture,
    Copy,
    Close,
    HomePage,
    Modify,
    NewScreenshot,
    Save,
    SelectArea,
    SelectFullscreen,
    Undo,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Capture => String::from("Capture"),
            Action::Close => String::from("Close"),
            Action::Copy => String::from("Copy"),
            Action::HomePage => String::from("HomePage"),
            Action::Modify => String::from("Modify"),
            Action::NewScreenshot => String::from("NewScreenshot"),
            Action::Save => String::from("Save"),
            Action::SelectArea => String::from("SelectArea"),
            Action::SelectFullscreen => String::from("SelectFullscreen"),
            Action::Undo => String::from("Undo"),
        }
    }
}