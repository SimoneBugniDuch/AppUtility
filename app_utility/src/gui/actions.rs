#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Capture,
    Copy,
    Close,
    HomePage,
    ManageTimer,
    Modify,
    NewScreenshot,
    ResetTimer,
    Save,
    SelectArea,
    SelectFullscreen,
    Settings,
    SetTimer,
    StartTimer,
    Undo,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Capture => String::from("Capture"),
            Action::Close => String::from("Close"),
            Action::Copy => String::from("Copy"),
            Action::HomePage => String::from("HomePage"),
            Action::ManageTimer => String::from("ManageTimer"),
            Action::Modify => String::from("Modify"),
            Action::NewScreenshot => String::from("NewScreenshot"),
            Action::ResetTimer => String::from("ResetTimer"),
            Action::Save => String::from("Save"),
            Action::SelectArea => String::from("SelectArea"),
            Action::SelectFullscreen => String::from("SelectFullscreen"),
            Action::SetTimer => String::from("SetTimer"),
            Action::StartTimer => String::from("StartTimer"),
            Action::Settings => String::from("Settings"),
            Action::Undo => String::from("Undo"),
        }
    }

    pub fn can_be_performed_during_image_view(&self) -> bool {
        match self {
            Action::Capture => false,
            Action::Close => true,
            Action::Copy => true,
            Action::HomePage => false,
            Action::ManageTimer => false,
            Action::Modify => true,
            Action::NewScreenshot => true,
            Action::ResetTimer => false,
            Action::Save => true,
            Action::SelectArea => false,
            Action::SelectFullscreen => false,
            Action::Settings => false,
            Action::SetTimer => false,
            Action::StartTimer => false,
            Action::Undo => true,
        }
    }
}