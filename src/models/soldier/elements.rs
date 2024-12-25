use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Element {
    Earth,
    #[default]
    Air,
    Fire,
    Water,
    Frost,
    Cosmic,
    Force,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Earth => write!(f, "Earth"),
            Element::Air => write!(f, "Air"),
            Element::Fire => write!(f, "Fire"),
            Element::Water => write!(f, "Water"),
            Element::Frost => write!(f, "Frost"),
            Element::Cosmic => write!(f, "Cosmic"),
            Element::Force => write!(f, "Force"),
        }
    }
}
