#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HomeMenuItem {
    Arm = 0,
    History = 1,
    Battery = 2,
}

impl HomeMenuItem {
    pub fn next(self) -> Self {
        match self {
            Self::Arm => Self::History,
            Self::History => Self::Battery,
            Self::Battery => Self::Arm,
        }
    }
}

pub fn home_lines(selected: HomeMenuItem) -> [&'static str; 4] {
    match selected {
        HomeMenuItem::Arm => ["Sentinel", "> Arm", "  History", "  Battery"],
        HomeMenuItem::History => ["Sentinel", "  Arm", "> History", "  Battery"],
        HomeMenuItem::Battery => ["Sentinel", "  Arm", "  History", "> Battery"],
    }
}
