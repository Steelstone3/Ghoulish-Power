use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum GhoulType {
    #[default]
    Undead,
    Light,
    Dark,
    Death,
    Soulless,
}

impl Display for GhoulType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhoulType::Undead => write!(f, "Undead"),
            GhoulType::Light => write!(f, "Light"),
            GhoulType::Dark => write!(f, "Dark"),
            GhoulType::Death => write!(f, "Death"),
            GhoulType::Soulless => write!(f, "Soulless"),
        }
    }
}
