#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Boot,
    Home,
    Armed,
    Alarm,
    History,
    Battery,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Boot
    }
}
