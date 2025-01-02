use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Default, RandGen)]
pub enum GhoulType {
    Dark,
    Death,
    Light,
    Soulless,
    #[default]
    Undead,
}

impl Display for GhoulType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhoulType::Dark => write!(f, "Dark"),
            GhoulType::Death => write!(f, "Death"),
            GhoulType::Light => write!(f, "Light"),
            GhoulType::Soulless => write!(f, "Soulless"),
            GhoulType::Undead => write!(f, "Undead"),
        }
    }
}

#[cfg(test)]
mod ghoul_type_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(GhoulType::Dark, "Dark")]
    #[case(GhoulType::Death, "Death")]
    #[case(GhoulType::Light, "Light")]
    #[case(GhoulType::Soulless, "Soulless")]
    #[case(GhoulType::Undead, "Undead")]
    fn display(#[case] ghoul_type: GhoulType, #[case] expected_display: &str) {
        // Then
        assert_eq!(expected_display, ghoul_type.to_string())
    }
}
