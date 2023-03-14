#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    Safety,
    Caution,
    Warning,
    Danger,
}

impl AlertLevel {
    pub fn id(&self) -> i8 {
        match self {
            AlertLevel::Safety => 1,
            AlertLevel::Caution => 2,
            AlertLevel::Warning => 3,
            AlertLevel::Danger => 4,
        }
    }
}
